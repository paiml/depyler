# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpW4g3RD/gQ_f8hH_XP.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
