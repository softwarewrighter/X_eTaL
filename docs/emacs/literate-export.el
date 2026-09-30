;;; literate-export.el --- export a literate document to HTML  -*- lexical-binding: t; -*-

;; Used by scripts/literate-html.sh in a batch Emacs:
;;   emacs --batch -Q -l docs/emacs/literate-export.el FILE.org
;; Blocks are not run again: the results recorded in the file (by
;; scripts/literate.sh) are exported as they are.

(require 'ox-html)
(require 'seq)
(require 'subr-x)

(setq org-export-use-babel nil
      org-export-time-stamp-file nil
      org-html-htmlize-output-type nil
      org-html-validation-link nil
      org-html-head-include-default-style nil
      org-html-head-include-scripts nil
      org-html-head "<link rel=\"stylesheet\" href=\"style.css\"/>"
      org-html-postamble t
      org-html-postamble-format
      '(("en" "<p><a href=\"index.html\">Literate documents</a> &middot; <a href=\"../\">Live demo</a> &middot; <a href=\"https://github.com/softwarewrighter/X_eTaL\">Repository</a></p>")))

;; A xetal block is shown drawn, as the live demo draws it (`xetal
;; render --html`, the binary in XETAL_BIN), with the lines as typed
;; after it as comments; other blocks export as usual.

(defvar literate-export-xetal (or (getenv "XETAL_BIN") "xetal"))

(defun literate-export--drawn (code)
  "CODE drawn as HTML spans by `xetal render --html'."
  (with-temp-buffer
    (let ((status (call-process literate-export-xetal nil t nil
                                "render" "--html" "-e" code)))
      (unless (eq status 0)
        (error "xetal render --html failed: %s" (buffer-string)))
      (string-trim-right (buffer-string)))))

(defun literate-export--typed (code)
  "The code lines of CODE as typed, as drawn comments (lamp, then the text)."
  (let* ((lamp (string #x235D))
         (lines (seq-remove (lambda (l) (string-match-p "\\`[ \t]*\\(#.*\\)?\\'" l))
                            (split-string code "\n"))))
    (mapconcat #'identity
               (cons (format "<span class=\"c-comment typed\">%s typed:</span>" lamp)
                     (mapcar (lambda (l)
                               (format "<span class=\"c-comment typed\">%s   %s</span>"
                                       lamp (org-html-encode-plain-text l)))
                             lines))
               "\n")))

(defun literate-export-src-block (src-block contents info)
  "A xetal SRC-BLOCK drawn, with what is typed as comments after it."
  (if (not (equal (org-element-property :language src-block) "xetal"))
      (org-html-src-block src-block contents info)
    (let ((code (string-trim-right (org-element-property :value src-block))))
      (format "<div class=\"org-src-container\">\n<pre class=\"src src-xetal\">%s\n\n%s</pre>\n</div>"
              (literate-export--drawn code)
              (literate-export--typed code)))))

(org-export-define-derived-backend 'literate-html 'html
  :translate-alist '((src-block . literate-export-src-block)))

(dolist (file command-line-args-left)
  (with-current-buffer (find-file-noselect file)
    ;; Org names headings with random ids: seed them, so an export
    ;; changes only when the document does.
    (random (file-name-nondirectory file))
    (org-export-to-file 'literate-html
      (concat (file-name-sans-extension file) ".html"))))
(setq command-line-args-left nil)

;;; literate-export.el ends here
