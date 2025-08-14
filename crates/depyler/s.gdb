# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpthf5sp/s.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
