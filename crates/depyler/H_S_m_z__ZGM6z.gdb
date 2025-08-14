# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp8UPXGb/H_S_m_z__ZGM6z.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
