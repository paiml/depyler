# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpfrxgTM/OUe3xxLkHUc5o.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
