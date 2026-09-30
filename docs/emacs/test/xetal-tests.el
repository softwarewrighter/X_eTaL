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

(ert-deftest xetal-mode-colours-each-kind ()
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

(provide 'xetal-tests)
;;; xetal-tests.el ends here
