# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpj4SnNU/KR_T8H_KI__p.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
