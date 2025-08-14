# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpNHjU3U/m_kDB__u56.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
