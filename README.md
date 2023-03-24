# Bestia pod grunwaldem
Chess engine template made by Krzysztof Łukasiewicz \
Last update: 28.02.2023 22:20

Implements partially standard UCI:
https://gist.github.com/aliostad/f4470274f39d29b788c1b09519e67372

requirements: \
Python version >= 3.10.0 \
python-chess

# How to run on Linux

Download and install some UCI-compatible GUI, e.g. http://www.playwitharena.de/. \
Replace in the file main.py line ```!/path/to/good/interpreter``` with the path to the python interpeter that you would like to use. \
Give executable rights to the file main.py with
```console
chmod +x main.py
```

Open GUI. \
Click Engines -> Manage... \
![Engine Management window](./images/manage.png)

and then Details. \
Click new and set the name and the path to the executable. \
![Engine Details window](./images/engine_settings.png) \
Eventually click "Start this engine now!". \
The engine should be loaded when you start a new game.

# How to run on Windows

Install Windows Subsystem for Linux \
https://learn.microsoft.com/en-us/windows/wsl/install \
and follow the steps for Linux

# Quick architecture overview

![Process architecture](./images/process_diagram.png) 

## UCI part
Input parser process doesn't understand UCI protocol, but it can interpret strings and send commands to UCI Protocol process. \
Output parser process also doesn't understand UCI protocol, but it can interpret UCI Protocol commands into respective strings. \
UCI Protocol process manages communication via Input parser process and Output parser process. Inteface between those processes is defined in https://github.com/marekpiotradamczyk/bestiapodgrunwaldem/blob/master/parser_commands.py

## Engine part

Engine process is the middleman between UCI Protocol process and Search process. \
Interface Engine <-> UCI is defined in https://github.com/marekpiotradamczyk/bestiapodgrunwaldem/blob/master/uci_commands.py \
Interface Engine <-> Search is defined in https://github.com/marekpiotradamczyk/bestiapodgrunwaldem/blob/master/search_commands.py