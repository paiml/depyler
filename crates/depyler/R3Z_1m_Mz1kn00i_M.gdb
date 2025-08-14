# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpPmYEMb/R3Z_1m_Mz1kn00i_M.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
