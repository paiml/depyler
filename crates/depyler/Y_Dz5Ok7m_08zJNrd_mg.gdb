# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpNsZeTA/Y_Dz5Ok7m_08zJNrd_mg.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
