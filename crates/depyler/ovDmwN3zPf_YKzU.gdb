# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpfNZIAm/ovDmwN3zPf_YKzU.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
