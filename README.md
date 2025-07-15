    Distributed Systems: Large Assignment 1 code{white-space: pre-wrap;} span.smallcaps{font-variant: small-caps;} span.underline{text-decoration: underline;} div.column{display: inline-block; vertical-align: top; width: 50%;} div.hanging-indent{margin-left: 1.5em; text-indent: -1.5em;} ul.task-list{list-style: none;} pre > code.sourceCode { white-space: pre; position: relative; } pre > code.sourceCode > span { display: inline-block; line-height: 1.25; } pre > code.sourceCode > span:empty { height: 1.2em; } code.sourceCode > span { color: inherit; text-decoration: inherit; } div.sourceCode { margin: 1em 0; } pre.sourceCode { margin: 0; } @media screen { div.sourceCode { overflow: auto; } } @media print { pre > code.sourceCode { white-space: pre-wrap; } pre > code.sourceCode > span { text-indent: -5em; padding-left: 5em; } } pre.numberSource code { counter-reset: source-line 0; } pre.numberSource code > span { position: relative; left: -4em; counter-increment: source-line; } pre.numberSource code > span > a:first-child::before { content: counter(source-line); position: relative; left: -1em; text-align: right; vertical-align: baseline; border: none; display: inline-block; -webkit-touch-callout: none; -webkit-user-select: none; -khtml-user-select: none; -moz-user-select: none; -ms-user-select: none; user-select: none; padding: 0 4px; width: 4em; color: #aaaaaa; } pre.numberSource { margin-left: 3em; border-left: 1px solid #aaaaaa; padding-left: 4px; } div.sourceCode { } @media screen { pre > code.sourceCode > span > a:first-child::before { text-decoration: underline; } } code span.al { color: #ff0000; font-weight: bold; } /\* Alert \*/ code span.an { color: #60a0b0; font-weight: bold; font-style: italic; } /\* Annotation \*/ code span.at { color: #7d9029; } /\* Attribute \*/ code span.bn { color: #40a070; } /\* BaseN \*/ code span.bu { } /\* BuiltIn \*/ code span.cf { color: #007020; font-weight: bold; } /\* ControlFlow \*/ code span.ch { color: #4070a0; } /\* Char \*/ code span.cn { color: #880000; } /\* Constant \*/ code span.co { color: #60a0b0; font-style: italic; } /\* Comment \*/ code span.cv { color: #60a0b0; font-weight: bold; font-style: italic; } /\* CommentVar \*/ code span.do { color: #ba2121; font-style: italic; } /\* Documentation \*/ code span.dt { color: #902000; } /\* DataType \*/ code span.dv { color: #40a070; } /\* DecVal \*/ code span.er { color: #ff0000; font-weight: bold; } /\* Error \*/ code span.ex { } /\* Extension \*/ code span.fl { color: #40a070; } /\* Float \*/ code span.fu { color: #06287e; } /\* Function \*/ code span.im { } /\* Import \*/ code span.in { color: #60a0b0; font-weight: bold; font-style: italic; } /\* Information \*/ code span.kw { color: #007020; font-weight: bold; } /\* Keyword \*/ code span.op { color: #666666; } /\* Operator \*/ code span.ot { color: #007020; } /\* Other \*/ code span.pp { color: #bc7a00; } /\* Preprocessor \*/ code span.sc { color: #4070a0; } /\* SpecialChar \*/ code span.ss { color: #bb6688; } /\* SpecialString \*/ code span.st { color: #4070a0; } /\* String \*/ code span.va { color: #19177c; } /\* Variable \*/ code span.vs { color: #4070a0; } /\* VerbatimString \*/ code span.wa { color: #60a0b0; font-weight: bold; font-style: italic; } /\* Warning \*/ body { font-family: Helvetica, sans; background-color: #f0f0f0; font-size: 12pt; color: black; text-decoration: none; font-weight: normal; } section.content { width: 19cm; font-size: 12pt; text-align: justify; margin-left: auto; margin-right: auto; margin-top: 20pt; background-color: white; padding: 20pt; } h1 { font-size: xx-large; text-decoration: none; font-weight: bold; text-align: center; } h2 { font-size: xx-large; text-decoration: none; font-weight: bold; text-align: left; border-bottom: 1px solid #808080; } h3 { font-size: x-large; text-decoration: none; font-weight: bold; text-align: left; } h1 + h3 { text-align: center; } h4 { font-size: large; text-decoration: none; font-weight: bold; text-align: left; } h5 { font-size: medium; text-decoration: none; font-weight: bold; text-align: left; } h6 { font-size: medium; text-decoration: none; font-weight: normal; text-align: left; } table { border-width: 1px; border-spacing: 0px; border-style: solid; border-color: #808080; border-collapse: collapse; font-family: Times, serif; font-size: 12pt; color: black; text-decoration: none; font-weight: normal; background-color: white; } td { border-width: 1px; border-style: solid; border-color: #808080; padding: 3pt; background-color: white; } th { border-width: 1px; border-style: solid; border-color: #808080; padding: 3pt; font-weight: bold; background-color: #f0f0f0; } a:link { color: blue; text-decoration: none; font-weight: normal; } a:visited { color: blue; text-decoration: none; font-weight: normal; } a:hover { text-decoration: underline; font-weight: normal; } pre.sourceCode { font-size: 90%; }

# Distributed Systems: Large Assignment 1

### Module system

Your task is to implement a module system supporting arbitrary user-provided types of modules and messages. The solution shall take the form of a library and follow the asynchronous programming paradigm. A template and public tests are provided in [this package](./dsassignment1.tgz).

The module system is a bit different from the executor system described in [Lab 03](../L03/). For details, see the Aditional Requirements section.

### Module system specification

Your solution shall have the same public interface as the one provided in `solution/lib.rs`, except for adding fields to the `TimerHandle`, `System`, and `ModuleRef` structs. You can add private items and change the file structure or split it into multiple files.

The module system interface provides the following functionality:

*   Creating and starting new instances of the system (`System::new()`).
    
*   Registering modules in the system (`System::register_module()`). The `Module` trait specifies bounds that must be satisfied by a module. Registering a module yields a `ModuleRef`, which can be then used to send messages to the module.
    
*   Sending messages to registered modules (`ModuleRef::send()`). A message of type `M` can be sent to a module of type `T` if `T` implements the `Handler<M>` trait. A module should handle messages in the order in which it receives them.
    
    A message is considered as delivered after the corresponding `ModuleRef::send()` has finished. In other words, the system must behave **as if** `ModuleRef::send()` inserted a message at the end of the receiving module’s message queue.
    
*   Creating new references to registered modules (`<ModuleRef as Clone>::clone()`).
    
*   Scheduling a message to be sent to a registered module periodically with a given interval (`ModuleRef::request_tick()`). The first tick should be sent after the interval elapsed. Requesting a tick yields a `TimerHandle`, which can be used to stop the sending of further ticks resulting from this request (`TimerHandle::stop()`).
    
    `ModuleRef::request_tick()` can be called multiple times. Every call results in sending more ticks and does not cancel ticks resulting from previous calls. For example, if `ModuleRef::request_tick()` is called at time 0 with interval 2 and at time 1 with interval 3, ticks should arrive at times 2, 4, 4 (two ticks at time 4), 6, 7, …
    
*   Shutting the system down gracefully (`System::shutdown()`). The shutdown should wait for all already started handlers to finish and for all registered modules to be dropped. It should not wait for all enqueued messages to be handled. It does not have to wait for all Tokio tasks to finish, but it must cause all of them to finish (e.g., it is acceptable if a task handling `ModuleRef::request_tick()` finishes an interval after the shutdown).
    
    It is undefined what happens when the system is used after a shutdown. However, you must ensure that a shutdown will not cause any panics in handlers or Tokio tasks (e.g., if a handler is already running when `System::shutdown()` is called, calls to `ModuleRef::send()` in that handler must not panic).
    

The `Message` trait defines what is expected from messages in the module system. It must be also possible to use `ModuleRef` as `Message`.

The `public-tests/tests/modules.rs` file contains an example of how the module system can be used. In the example there are two modules: `Ping` and `Pong`. After being registered in the system, they are sent references to each other so that they can communicate. Then, one of the modules sends a `Ball` message to the other module. That module replies to it with a new `Ball` message, to which the first module replies with the next `Ball` message, and so forth.

#### Additional requirements

Your solution should be **asynchronous** and allow multiple modules to execute **concurrently**. It will be run using Tokio. Keep in mind you can run as many Tokio tasks as you want.

In this assignment, you are **not** supposed to implement an executor system like the one from [Lab 3](../L03/). In particular:

*   You should not create any threads or Tokio runtimes. You solution will be run in an already existing Tokio runtime.
*   You should not use a system-wide message queue for all modules or a system-wide loop for executing all modules’ handlers.

You do not need to remove a module from the system when its last `ModuleRef` is dropped.

Ticks requested by `System::request_tick()` must be delivered at specified time intervals. There shall be no drift, that is, the difference between the expected and actual number of ticks sent so far in any moment after `ModuleRef::request_tick()` is called must be bounded.

The module system must not panic unless a registered module panics (because of a user-provided message handler).

### Varia

You can use logging if you want to, but do not emit a large amount of logs at levels `>= INFO` when the system is operating properly. All logging must be done via the `log` crate.

You can only use crates specified in the provided `Cargo.toml` file.

### Hints

You might encounter the following problem: what type to use for “some message that can be handled by a module of type `T`”? One way of dealing with this issue is to introduce a private helper trait:

```
#[async_trait::async_trait]
trait Handlee<T: Module>: Message {
    async fn get_handled(self: Box<Self>, module_ref: &ModuleRef<T>, module: &mut T);
}

#[async_trait::async_trait]
impl<M: Message, T: Handler<M>> Handlee<T> for M {
    async fn get_handled(self: Box<Self>, module_ref: &ModuleRef<T>, module: &mut T) {
        module.handle(module_ref, *self).await;
    }
}
```

You can then use a trait object of type `Box<dyn Handlee<T>>`. The traits `Handler` and `Handlee` are related in a way similar to `From` and `Into`. An important difference is that here we use `self: Box<Self>`. Thanks to that when calling `get_handled()` we can move the box rather than the boxed value. The latter would be impossible, because in `Box<dyn Handlee<T>>` the boxed value has a statically unknown size.

### Testing

You are given a subset of official tests. Their intention is to make sure that the public interface of your solution is correct, and to evaluate basic functionality.

Your solution will be tested with the stable Rust version `rustc 1.82.0 (f6e511eec 2024-10-15)`.

### Grading

Your solution will be graded based on results of automatic tests and code inspection. The number of available and required points is specified in the [Passing Rules](../../) described at the main website of the course. If your solution passes the public tests, you will receive at least the required number of points.

### Asking questions

Questions **must** be asked on a dedicated Moodle forum. This way everybody will be able to read the answers. Try to ask questions early if there are any. We will try not to require any changes to existing solutions when providing answers.

### Submitting solution

Your solution must be submitted as a single `.zip` file with its name being your login at students (e.g., `ab123456.zip`). After unpacking the archive, a directory path named `ab123456/solution/` must be created. In the `solution` subdirectory there must be a Rust library crate that implements the required interface. Project `public-tests` must be able to be built and tested cleanly when placed next to the `solution` directory.

- - -

Authors: F. Plata, K. Iwanicki, M. Banaszek, W. Ciszewski, M. Matraszek., Distributed Systems Course (fall 2024/2025) at UW
