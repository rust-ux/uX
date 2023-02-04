# uX maintainence manual

The project maintainers are the users designated as owners of the project's GitHub organization.

We use pull requests to review changes to the master branch.
Generally, before merging, pull requests should be reviewed by a maintainer
who is not also the author of the PR.

## Development goals

* Basic properties of the types in uX are advertised already in the README:
  Behave as close as possible to the corresponding builtin type,
  be stored like them,
  and give the compiler as much information as practical to work with them
  (eg. advertising niches).
* Builtin types often have traits implemented for them in third party crates.
  We encourage these third parties to optionally depend on uX
  and implement the traits for these types as well;
  when that is impossible for some reason,
  we may add an optional dependency and implement the traits here instead.
* On the long run,
  this crate's goal is to be obsoleted by ranged integers in the core library.
  When that happens
  (and there is at least [an issue](https://github.com/rust-lang/rfcs/issues/671) open),
  this crate can be demoted to a compatibility wrapper.
