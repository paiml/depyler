# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpuCXJAy/pt6Q.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
