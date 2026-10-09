;;; xetal-tests.el --- ERT tests for xetal-mode and ob-xetal  -*- lexical-binding: t; coding: utf-8 -*-

;; Copyright (c) 2026 Michael A Wright
;; SPDX-License-Identifier: MIT

;;; Commentary:

;; Run by `just test-emacs':
;;
;;   emacs --batch -Q -L docs/emacs -l docs/emacs/test/xetal-tests.el \
;;         -f ert-run-tests-batch-and-exit
;;
;; The babel tests run the xetal in XETAL_BIN, or target/debug/xetal.

;;; Code:

(require 'ert)
(require 'org)
(require 'xetal-mode)
(require 'ob-xetal)

(setq org-babel-xetal-command
      (or (getenv "XETAL_BIN")
          (expand-file-name "../../../target/debug/xetal"
                            (file-name-directory (or load-file-name buffer-file-name)))))
(setq org-confirm-babel-evaluate nil)
(org-babel-do-load-languages 'org-babel-load-languages '((xetal . t)))

(defun xetal-tests--face-at (text needle)
  "The face xetal-mode gives the first NEEDLE in TEXT."
  (with-temp-buffer
    (insert text)
    (xetal-mode)
    (font-lock-ensure)
    (goto-char (point-min))
    (search-forward needle)
    (let ((face (get-text-property (match-beginning 0) 'face)))
      (if (consp face) (car face) face))))

(ert-deftest xetal-mode-tuples-and-patterns ()
  ;; TU1, TU4: the comma and the wildcard are plain text, never a name.
  (let ((line "(w, _) := (u:s_tep s, 1 2)"))
    (should (eq (xetal-tests--face-at line "u:s_tep") 'xetal-user-face))
    (should (null (xetal-tests--face-at line ",")))
    (should (null (xetal-tests--face-at line "_)")))
    (should (eq (xetal-tests--face-at line "1") 'xetal-number-face))))

(ert-deftest xetal-mode-colors-each-kind ()
  (let ((line "u:s_q := { _r * _r } '+ r_/ c:K_ 3 \"s:\" u_se< \"Stats\" # note"))
    (should (eq (xetal-tests--face-at line "u:s_q") 'xetal-user-face))
    (should (eq (xetal-tests--face-at line "_r") 'xetal-argument-face))
    (should (eq (xetal-tests--face-at line "*") 'xetal-symbol-face))
    (should (eq (xetal-tests--face-at line "r_/") 'xetal-builtin-face))
    (should (eq (xetal-tests--face-at line "c:K_") 'xetal-library-face))
    (should (eq (xetal-tests--face-at line "3") 'xetal-number-face))
    (should (eq (xetal-tests--face-at line "u_se<") 'xetal-macro-face))
    (should (eq (xetal-tests--face-at line "Stats") 'font-lock-string-face))
    (should (eq (xetal-tests--face-at line "note") 'font-lock-comment-face))))

(ert-deftest xetal-mode-colors-long-prefixes ()
  (let ((line "combinators:K_ 1 2 b2:m_ean 3"))
    (should (eq (xetal-tests--face-at line "combinators:K_") 'xetal-library-face))
    (should (eq (xetal-tests--face-at line "b2:m_ean") 'xetal-library-face))))

(ert-deftest xetal-mode-shows-glyphs-but-keeps-ascii ()
  (with-temp-buffer
    (insert "u:s_ub := { _l - _r }")
    (xetal-mode)
    (font-lock-ensure)
    (should prettify-symbols-mode)
    (should (equal (buffer-string) "u:s_ub := { _l - _r }"))
    (goto-char (point-min))
    (search-forward ":=")
    (should (get-text-property (match-beginning 0) 'composition))))

(defun xetal-tests--run (org)
  "Execute every block of the Org text ORG; the buffer afterwards."
  (with-temp-buffer
    (insert org)
    (org-mode)
    (org-babel-execute-buffer)
    (buffer-string)))

(ert-deftest ob-xetal-runs-a-block ()
  (should (string-match-p "^: 6$" (xetal-tests--run "#+begin_src xetal\n'+ r_/ 1 2 3\n#+end_src\n"))))

(ert-deftest ob-xetal-session-continues-earlier-blocks ()
  (let ((out (xetal-tests--run (concat "#+begin_src xetal :session s\nu:s_q := { _r * _r }\nu:s_q 3\n#+end_src\n\n"
                                       "#+begin_src xetal :session s\nu:s_q 4\n#+end_src\n"))))
    (should (string-match-p "^: 9$" out))
    (should (string-match-p "^: 16$" out))
    (should-not (string-match-p "^: 9\n: 16$" out))))

(ert-deftest ob-xetal-seed-fixes-the-rolls ()
  (let ((block "#+begin_src xetal :seed 3\nr_oll! 100 100 100\n#+end_src\n"))
    (should (equal (xetal-tests--run block) (xetal-tests--run block)))))

(ert-deftest ob-xetal-echo-shows-the-statement ()
  (should (string-match-p "^: 3$" (xetal-tests--run "#+begin_src xetal :echo yes\n1 + 2\n#+end_src\n"))))

(ert-deftest ob-xetal-reports-a-failing-block ()
  (should-error (xetal-tests--run "#+begin_src xetal\n1 / 0\n#+end_src\n") :type 'user-error))

;; Pictures: a block with `:results file :file PATH` saves the last
;; picture it shows ([]S_HOW) to PATH, and its result is a link to it.

(defun xetal-tests--in-dir (org)
  "Execute the blocks of ORG in a fresh directory; (buffer . dir)."
  (let ((dir (make-temp-file "xetal-tests-" t)))
    (with-temp-buffer
      (setq default-directory (file-name-as-directory dir))
      (insert org)
      (org-mode)
      (org-babel-execute-buffer)
      (cons (buffer-string) dir))))

(defun xetal-tests--file (dir name)
  "The text of the file NAME in DIR."
  (with-temp-buffer
    (insert-file-contents (expand-file-name name dir))
    (buffer-string)))

(ert-deftest ob-xetal-file-saves-the-picture-and-links-it ()
  (let* ((run (xetal-tests--in-dir
               "#+begin_src xetal :results file :file pics/one.svg\np := []S_HOW []G_RID 1 0\n#+end_src\n"))
         (svg (xetal-tests--file (cdr run) "pics/one.svg")))
    (should (string-match-p "\\[\\[file:pics/one.svg\\]\\]" (car run)))
    (should (string-prefix-p "<svg " svg))
    (should (string-match-p "width=\"48\"" svg))))

(ert-deftest ob-xetal-file-takes-the-blocks-own-picture-in-a-session ()
  (let* ((run (xetal-tests--in-dir
               (concat "#+begin_src xetal :session s\na := []S_HOW []G_RID 1 1 1\nn := 4\n#+end_src\n\n"
                       "#+begin_src xetal :session s :results file :file b.svg\nb := []S_HOW []G_RID n r_eshape 1\n#+end_src\n")))
         (svg (xetal-tests--file (cdr run) "b.svg")))
    (should (string-match-p "width=\"96\"" svg))))

(ert-deftest ob-xetal-file-without-a-picture-is-an-error ()
  (should-error (xetal-tests--in-dir "#+begin_src xetal :results file :file x.svg\n1 + 2\n#+end_src\n")
                :type 'user-error))

(provide 'xetal-tests)
;;; xetal-tests.el ends here
