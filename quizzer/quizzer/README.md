# Quizzer

This exercise is about both design and finding information. You'll have to
figure out a model to represent your quiz questions, as well as a means to store
them into a JSON file, and load them yourself. Also, you will have to find out
how to parse the program arguments.

We will use the project we just set up to write a quiz game creator and player.
You may add other dependencies as needed.

It has the following functional requirements:

- It runs as a command-line tool in your terminal.
- It has two modes: question-entering mode quiz mode.
- The mode is selected with a subcommand, passed as the first argument to the
  program.
- Question-entering mode: Allows for entering multiple-choice quiz questions,
  with 4 possible answers each, exactly 1 of them being correct. The questions
  are stored on disk as a JSON file.
- Quiz mode: Loads stored questions from the JSON file, presents the questions
  one-by-one to the player, reads and verifies the player input, and presents
  the score at the end of the game.
- Errors are correctly handled, i.e. your application does not panic if it
  encounters any unexpected situation.
- Use anywhow and the question-mark (?) operator to make error-bubbling concise.

## structure

- Logic concerning creating, storing, and loading quiz questions is defined in
  the library part of your crate.
- Functionality regarding user input (arg parsing, reading from stdin) is
  defined in the application code, not in your library.
- Logical units of your crate are divided up into modules.

## tip

Before you start coding, make sure you've listed all open questions and found
answers to them. You're also encouraged to draw a simple diagram of the module
structure of your application, annotating each module with its responsibilities.
