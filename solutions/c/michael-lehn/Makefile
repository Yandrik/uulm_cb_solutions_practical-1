Project = tr
#CC = gcc-11
CFLAGS = -Wall
RANLIB = ranlib

Src = $(filter-out _gen_%, $(wildcard *.c))
Header = $(filter-out _gen_%, $(wildcard *.h))
DepsDir = deps
DependFiles = $(patsubst %.c,%.d,$(LibrarySrcC) $(TargetsSrc))

TargetSrc = $(filter x%, $(Src))
TargetObj = $(patsubst %.c,%.o,$(TargetSrc))
Target    = $(patsubst %.c,%,$(TargetSrc))

GenSrc = $(wildcard _*.c)
Gen    = $(patsubst %.c,%,$(GenSrc))

GenFile = 

LibSrc = $(filter-out $(TargetSrc) $(GenSrc), $(Src))
LibObj = $(patsubst %.c,%.o,$(LibSrc))

Lib         = lib$(Project).a
LibContent  = $(if $(wildcard $(Lib)),$(shell ar t $(Lib) | grep -v "^__"),)
LibRemoves  = $(filter-out $(LibObj),$(LibContent))
SrcRemoves  = $(patsubst %.o,%.c,$(LibRemoves))
ArDelete    = $(if $(LibRemoves),ar d $(Lib) $(LibRemoves),)

.PHONY: all clean
all: $(Target) $(Gen) $(Lib)

$(GenFile) : token.txt $(Gen)
	./ygen

$(Lib)(%.o) : %.o
	$(AR) cr $@ $^

$(SrcRemoves) :
	$(ArDelete)

$(Lib) : $(Lib)($(LibObj)) $(SrcRemoves)
	$(RANLIB) $(Lib)

$(DepsDir):
	mkdir $(DepsDir)

$(Target): % : %.o $(Lib)
	$(CC) $(CFLAGS) -o $@ $^

clean:
	$(RM) $(DependFiles)
	$(RM) -r $(DepsDir)
	$(RM) $(LibObj) $(Lib)
	$(RM) $(TargetObj) $(Target) $(Gen)
	$(RM) $(GenFile)

$(DepsDir)/%.d: %.c | $(DepsDir) $(GenFile)
	@echo "Updating dependencies for $<"
	@set -e; $(RM) -f $@; \
	$(CC) -M $(CPPFLAGS) $< > $@.$$$$; \
	sed 's,\($*\)\.o[ :]*,\1.o $@ : ,g' < $@.$$$$ > $@; \
	$(RM) -f $@.$$$$

ifneq ($(MAKECMDGOALS), clean)
-include $(patsubst %,$(DepsDir)/%,$(TargetSrc:.c=.d) $(LibSrc:.c=.d))
endif
