# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpV34HU0/TaY9.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
