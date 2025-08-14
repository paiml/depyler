# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpldMikK/Kpvb1v_qh17qHRTQ5R_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
