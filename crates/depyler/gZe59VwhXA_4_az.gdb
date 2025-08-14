# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpwsI4Ba/gZe59VwhXA_4_az.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
