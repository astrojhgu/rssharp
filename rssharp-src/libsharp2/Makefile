all: lib/libsharp.a
py: pysharp.so


obj/%.o: src/%.c
	mkdir -p obj
	$(CC) $< -o $@ -c -I . -fPIC -g -O2

obj/%.o: src/%.cxx
	mkdir -p obj
	$(CXX) $< -o $@ -c -I . -fPIC -g -O2


lib/libsharp.a: obj/pocketfft.o obj/sharp_almhelpers.o obj/sharp.o obj/sharp_core.o obj/sharp_core_inc.o obj/sharp_geomhelpers.o obj/sharp_legendre_roots.o obj/sharp_utils.o obj/sharp_ylmgen_c.o
	mkdir -p lib
	$(AR) cr $@ $^ 
	


pysharp.so: py/pysharp.cc lib/libsharp.a
	$(CXX) -o $@ $< -I . `pkg-config --cflags python3` -L ./lib -lsharp  --shared -fPIC -O2

test: map2alm2map_test.cpp lib/libsharp.a
	g++ -o $@ $< -I . -L ./lib -lsharp -lm

test_spin: test_spin.cpp lib/libsharp.a
	g++ -o $@ $< -I . -L ./lib -lsharp -lm

clean: 
	$(RM) *.o
