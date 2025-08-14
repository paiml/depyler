# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpypJdce/F_DyCVL9.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
