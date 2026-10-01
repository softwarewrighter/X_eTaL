;;; ob-xetal.el --- Org Babel support for X_eTaL  -*- lexical-binding: t; coding: utf-8 -*-

;; Copyright (c) 2026 Michael A Wright
;; SPDX-License-Identifier: MIT

;;; Commentary:

;; Source blocks in X_eTaL, run by xetal:
;;
;;   #+begin_src xetal :session tour
;;   u:s_quare := { _r * _r }
;;   u:s_quare 1 2 3
;;   #+end_src
;;
;; A block without a session runs on its own (`xetal run').  With
;; `:session NAME' it continues the blocks above it in the buffer that
;; name the same session: they run first, silently, and only this
;; block's output is its result (`xetal run --context').  The session
;; is read from the buffer each time, so running a block again, or the
;; whole buffer, gives the same results.  The header arguments:
;;
;;   :session NAME   continue the earlier blocks of session NAME
;;   :seed N         fix the rolls of r_oll! (for reproducible results)
;;   :echo yes       show each statement decorated, then its output
;;                   (a notebook run; not with :session)
;;   :untyped yes    skip the type checker (not with :session)
;;   :results file :file PATH
;;                   save the last picture the block shows ([]S_HOW,
;;                   an SVG) to PATH; the result is a link to it, and
;;                   the HTML export shows the picture (animated, if
;;                   it has frames); the block's printed text is not
;;                   recorded
;;
;; A picture block, in a session or not:
;;
;;   #+begin_src xetal :session life :results file :file ../../images/literate-glider.svg
;;   torus := []S_HOW []G_RID 40 u:f_rames glider
;;   #+end_src
;;
;;   (add-to-list 'load-path "path/to/X_eTaL/docs/emacs")
;;   (require 'ob-xetal)
;;   (add-to-list 'org-babel-load-languages '(xetal . t))

;;; Code:

(require 'ob)
(require 'xetal-mode)

(defgroup ob-xetal nil "Org Babel support for X_eTaL." :group 'org-babel)

(defcustom org-babel-xetal-command "xetal"
  "The xetal program: a name on `exec-path', or a path."
  :type 'string
  :group 'ob-xetal)

(defvar org-babel-default-header-args:xetal
  '((:results . "output") (:session . "none"))
  "Default header arguments for xetal source blocks.")

(add-to-list 'org-src-lang-modes '("xetal" . xetal))
(with-eval-after-load 'ob-tangle
  (add-to-list 'org-babel-tangle-lang-exts '("xetal" . "xtl")))

(defun org-babel-xetal--program ()
  "The xetal to run: `org-babel-xetal-command' as a path or found on
`exec-path', else in ~/.local/bin or ~/.local/softwarewrighter/bin,
where a GUI Emacs with a short PATH would not look."
  (let ((command org-babel-xetal-command))
    (or (and (file-name-absolute-p command) (file-executable-p command) command)
        (executable-find command)
        (seq-find #'file-executable-p
                  (list (expand-file-name command "~/.local/bin")
                        (expand-file-name command "~/.local/softwarewrighter/bin")))
        (user-error "xetal block: %S not found; set `org-babel-xetal-command'" command))))

(defun org-babel-xetal--session (params)
  "The session PARAMS name, or nil for none."
  (let ((session (cdr (assq :session params))))
    (unless (member session '(nil "none")) session)))

(defun org-babel-xetal--context (session)
  "The bodies of the xetal blocks of SESSION above point, in order."
  (let ((here (point)) (bodies '()))
    (save-excursion
      (goto-char (point-min))
      (while (and (re-search-forward org-babel-src-block-regexp nil t)
                  (< (match-beginning 0) here))
        (let ((info (org-babel-get-src-block-info 'no-eval)))
          (when (and (equal (nth 0 info) "xetal")
                     (equal (org-babel-xetal--session (nth 2 info)) session))
            (push (nth 1 info) bodies)))))
    (mapconcat (lambda (b) (concat b "\n")) (nreverse bodies) "")))

(defun org-babel-xetal--arguments (params file context &optional draw)
  "The xetal command line for FILE with header arguments PARAMS and,
for a session, the CONTEXT file; pictures go to the directory DRAW."
  (let ((seed (cdr (assq :seed params)))
        (echo (equal (cdr (assq :echo params)) "yes"))
        (untyped (equal (cdr (assq :untyped params)) "yes")))
    (when (and context (or echo untyped))
      (user-error "xetal block: :echo and :untyped do not go with :session"))
    (append (list "run")
            (when draw (list "--draw" draw))
            (when seed (list "--seed" (format "%s" seed)))
            (when echo (list "--echo"))
            (when untyped (list "--untyped"))
            (when context (list "--context" context))
            (list file))))

(defun org-babel-xetal--write (file text)
  "Write TEXT to FILE as UTF-8."
  (with-temp-file file
    (set-buffer-file-coding-system 'utf-8-unix)
    (insert text)))

(defun org-babel-xetal--picture (draw target)
  "Move the last picture xetal wrote to the directory DRAW (NAME-1.svg,
NAME-2.svg, ...) to TARGET, making its directory."
  (let* ((number (lambda (f) (string-to-number
                              (replace-regexp-in-string "\\`.*-\\([0-9]+\\)\\.svg\\'" "\\1" f))))
         (pictures (sort (directory-files draw t "-[0-9]+\\.svg\\'")
                         (lambda (a b) (< (funcall number a) (funcall number b))))))
    (unless pictures
      (user-error "xetal block: :file %s, but the block shows no picture ([]S_HOW)" target))
    (make-directory (or (file-name-directory (expand-file-name target)) ".") t)
    (copy-file (car (last pictures)) (expand-file-name target) t)))

(defun org-babel-execute:xetal (body params)
  "Run BODY, an xetal block with header arguments PARAMS, and return
what it printed; a failing block shows its error. With :file, save
the block's last picture there instead and return nil (Org links it)."
  (let* ((session (org-babel-xetal--session params))
         (target (cdr (assq :file params)))
         (file (make-temp-file "ob-xetal-" nil ".xtl"))
         (context (and session (make-temp-file "ob-xetal-context-" nil ".xtl")))
         (draw (and target (make-temp-file "ob-xetal-draw-" t)))
         (program (org-babel-xetal--program)))
    (unwind-protect
        (progn
          (org-babel-xetal--write file (concat (org-babel-expand-body:generic body params) "\n"))
          (when context
            (org-babel-xetal--write context (org-babel-xetal--context session)))
          (with-temp-buffer
            (let* ((coding-system-for-read 'utf-8)
                   (status (apply #'call-process program nil t nil
                                  (org-babel-xetal--arguments params file context draw))))
              (unless (eq status 0)
                (user-error "xetal block: exit %s: %s" status (buffer-string)))
              (if target
                  (progn (org-babel-xetal--picture draw target) nil)
                (buffer-string)))))
      (delete-file file)
      (when context (delete-file context))
      (when draw (delete-directory draw t)))))

(provide 'ob-xetal)
;;; ob-xetal.el ends here
