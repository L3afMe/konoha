# Contribution Guidelines

Please note: any contributions will be licensed under
the repository license, which is the GNU GPL v3.0 license.

## Pull requests

* Format commit messages to follow the following convensions

    ```error-be-gone
    (Section): (Brief summary) (Optional: Issue number if related)

    (Optional: Longer summary on what was changed or any additional information)
    ```

* Sections
  * `bug` - Bug fixed, link to issue at the end if there is one open
  * `ci` - Anything GitHub Actions related
  * `docs` - Anything documentation related
  * `feat` - New feature added
  * `impr` - Improved upon something without adding additional features

* Valid examples
  * `feat: add popup config`
  * `bug: fix crash caused by menu size #1`
  * `ci: add format checking on commit/pr`
  * `impr: rewritten notification handler`

* Invalid examples
  * `Feat: add popup config`
  * `feat: Add popup config`
  * `feat: add popup config.`
  * `feat: fix crash caused by menu size #1`
  * `bug: add popup config`

