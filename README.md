# Dual Factor Authentication App

## Note
- This project is not meant to be cloned unless the cloner is willing to set up a separate email account for themselves. This repository is mainly for version control and documentation.

## Current State

- The app currently is able to send emails from my testauth account in Gmail.
- Has password protection for my backend that requires a separate 'password.txt' file in order to send.


## Typical Use case

1. User opens web page, logs in.
2. A simple verification email is sent to the inbox of the user
3. Link in the email verifies the session, and they make it to a final landing page

## Back end

1. Lettre for sending email.
2. Google Cloud for data storage.
3. [traxon99.github.io](http://traxon99.github.io) for log in website.

## Goals

- Learn file input/output in Rust.
- Learn about integrating external crates into projects.
- Learn how account creation and storage will work.
- Learn Cloud Computing integration for user database.
