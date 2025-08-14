# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpWMIDxA/T__7_h_E_ssF.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
