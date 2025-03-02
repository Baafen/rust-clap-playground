# rust-clap-playground
Testing out rust and clap by making a router like cli

As this is a playground nothing is really complete.

## Usage

*set* is used to configure an object.
Currently only interface has anything meaningful.
Set has no effect except that a successful command will print command values.

*edit* is used to change path to make set operate at a different level.
Current path is displayed on the prompt.
Note that the path is currently not validated causing set to fail.
Using edit when already on a path will go further down the path.
Unless edit is used without any arguments, then user is moved back to root.

Typing *exit* at any level will exit the program.

**Example**  
```
> edit interface eth0  
interface/eth0 > set address 192.168.0.1  
interface/eth0 > set description 'a description'  
```

Typing edit without a path will move back to root.
Above is equal to typing the following from root:

```
set interface eth0 address 192.168.0.1  
set interface eth0 description 'a description'  
```

To exit just type *exit*.
