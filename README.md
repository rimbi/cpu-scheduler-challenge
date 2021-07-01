### Coding Challenge Guidelines

We need a function that will evaluate the order that a set of tasks will be completed in.
When idle, the CPU will take the next task that has been queued with the lowest time to complete.

Tasks are queued at the moment in time given by `queued_at`.
Tasks will keep the CPU busy for `execution_duration` units of time.
`queued_at` and `execution_duration` are not in a particular unit of time, but you may consider them in seconds, milliseconds, or whatever makes sense to you.
The CPU can only execute 1 task at a time.

You will find some example test cases and the boilerplate for the function in src/lib.rs.
The test cases are not exhaustive for how the function should work, but should clarify desired behavior.
Please add additional ones.

You can use anything from Rust's std library.

### Evaluation Criteria

You will be evaluated on, in order of importance:

1. Creating a working solution to the test cases provided.
2. Creating a working solution for plausible inputs beyond the test cases provided.
3. Correct analysis of the runtime complexity of your solution (in questions.md).
4. Additional test cases (even if your solution does not pass them).
5. Code quality.
6. Efficient solution in terms of runtime complexity.
7. Answers to questions in questions.md.

Please make frequent commits, we would like to see your progress towards a solution (we understand and expect them to be messy).


### Useful Links

[std::collections](https://doc.rust-lang.org/std/collections/index.html)


### CodeSubmit

Please organize, design, test and document your code as if it were
going into production - then push your changes to the master branch.

All the best,

The CodeSubmit Team
