# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpRr8nGL/p7nr.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
