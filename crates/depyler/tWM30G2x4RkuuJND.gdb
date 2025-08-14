# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpConjji/tWM30G2x4RkuuJND.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
