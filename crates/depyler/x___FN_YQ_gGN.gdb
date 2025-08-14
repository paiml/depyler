# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpRdjmgv/x___FN_YQ_gGN.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
