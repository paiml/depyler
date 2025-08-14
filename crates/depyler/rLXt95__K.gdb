# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpqfxUQz/rLXt95__K.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
