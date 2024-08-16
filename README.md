# Rust CLI Football Manager

## Intro

This project simulates popular PC game Football Manager.

It uses CLI to execute commands that will change state in game text file. We use text files as permanent data storage. Main idea is to write all methods that program ran in the past with their specific arguments, so that on game restart we can start from correct point in time. This works exactly like WAL file that SQL databases use under the hood to ensure durability of transactions.
