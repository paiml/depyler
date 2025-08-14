# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpAjwd2y/IW6_2T.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
