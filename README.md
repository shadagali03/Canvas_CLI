<p align="center">
 <h1 align="center">Canvas CLI</h1>

  <p align="center">
   Command Line Interface tool that allows users to submit their assignments from the terminal
  </p>
</p>

## About The Project
As a computer science student I have spent a lot of time in the terminal, unfortunately I have also spent a lot of time working and submitting assignments.
I decided to implement a CLI tool that allows users to submit their assignments from the terminal. I modeled my project to how github works when pushing to a remote repo.
Note: This is a personal project and although others can use it is not meant for distrubution purposes and may contain security issues with you Canvas tokens.

Why Canvas_CLI
* Allows you to scan through your courses and assignemnts to see what needs to be completed
* Submit assignments similar to the way github works (git add, git commit, git push)
* Can reduce the time it needs to take to submit an assignment and may be more comfortable for nerdy kids like us :)

## Built With
For this project I used **Rust**. This was a big learning experience for me and my first Rust project.

## Usage
1. First clone the repo into your local machine
2. You will also need to add your Canvas Authentication Token **(Note this is a secure token and should not be distributed)**
3. Navigate to your Canvas account and go to Profile > Settings
4. Scroll to the bottom and click 'New Access Token'
5. Naviagate back to the terminal and run the program
6. login <school_url> <auth_token> will be the command you need to run and will sync your data so you wont have to do it again
7. You can finally submit your assignemnts through the terminal!

## Future Plans
I plan on refactoring the code quite a bit 
* Most issues reside in not being completely familiar with Rust.
* I would also like to implement the OAUTH version as opposed to manual authentication so this can actually be distributed
* Implement a database and encryption/decryption to safely handle user information

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch 
3. Commit your Changes
4. Push to the Branch
5. Open a Pull Request
