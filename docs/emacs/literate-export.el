;;; literate-export.el --- export a literate document to HTML  -*- lexical-binding: t; -*-

;; Used by scripts/literate-html.sh in a batch Emacs:
;;   emacs --batch -Q -l docs/emacs/literate-export.el FILE.org
;; Blocks are not run again: the results recorded in the file (by
;; scripts/literate.sh) are exported as they are.

(require 'ox-html)

(setq org-export-use-babel nil
      org-html-htmlize-output-type nil
      org-html-validation-link nil
      org-html-head-include-default-style nil
      org-html-head-include-scripts nil
      org-html-head "<link rel=\"stylesheet\" href=\"style.css\"/>"
      org-html-postamble t
      org-html-postamble-format
      '(("en" "<p><a href=\"index.html\">Literate documents</a> &middot; <a href=\"../\">Live demo</a> &middot; <a href=\"https://github.com/softwarewrighter/X_eTaL\">Repository</a></p>")))

(dolist (file command-line-args-left)
  (with-current-buffer (find-file-noselect file)
    ;; Org names headings with random ids: seed them, so an export
    ;; changes only when the document does.
    (random (file-name-nondirectory file))
    (org-html-export-to-html)))
(setq command-line-args-left nil)

;;; literate-export.el ends here
