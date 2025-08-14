# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpTwxV9J/LHTnGN7nNJG9V.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
