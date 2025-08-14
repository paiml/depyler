# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpipKwsh/m_4jeEJpFv87.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
