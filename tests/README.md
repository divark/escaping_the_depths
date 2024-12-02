# Summary
All features developed for this game start in this directory.

A feature is first fleshed out using a [Gherkin](https://cucumber.io/docs/gherkin/reference/) file, using plain english. These are found
in the `features` folder.

These feature files are then converted into an [Executable Specification](https://cucumber.io/blog/hiptest/what-are-executable-specifications/), where each step is
turned into a piece of code that ensures these requirements are being fulfilled. These
Executable Specification files are named similarly to each feature file found in the `features`
folder, with the only difference being that they hold rust code now.
