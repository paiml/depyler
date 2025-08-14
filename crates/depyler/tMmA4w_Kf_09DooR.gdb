# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpId9qpj/tMmA4w_Kf_09DooR.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
