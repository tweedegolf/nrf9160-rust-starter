/* Linker script for the nRF9160 in Non-secure mode */
MEMORY
{

    SPM                      : ORIGIN = 0x00000000, LENGTH = 320K
    FLASH                    : ORIGIN = 0x00050000, LENGTH = 704K
    RAM                      : ORIGIN = 0x20018000, LENGTH = 160K

}


SECTIONS
{
  /* Define an spm region where the compiler can put the spm for us */
  .spm :
  {
    KEEP(*(.spm .spm.*));
  } > SPM
}
