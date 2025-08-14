# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpvOyr6I/T39e2u__rkhIWaY11z_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
