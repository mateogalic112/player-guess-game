# Rust CLI Football Manager

## Intro

This project simulates popular PC game Football Manager.

It uses CLI to execute commands that will change game state in text file. We use simple plain text file as permanent data storage. Main idea is to write all method signatures that program has ran in the past with their specific arguments, so that on game restart we can play the game from correct point in time. This works exactly like WAL file that SQL databases use under the hood to ensure durability of transactions.
