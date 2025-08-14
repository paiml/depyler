# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpmwvfqF/Ysv9d__zA2LG4_W.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
