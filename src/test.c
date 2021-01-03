 static EFI_STATUS EFIAPI
 efi_get_memory_map_wrapper ( UINTN *memory_map_size,
                              EFI_MEMORY_DESCRIPTOR *memory_map, UINTN *map_key,
                              UINTN *descriptor_size,
                              UINT32 *descriptor_version ) {
         EFI_BOOT_SERVICES *bs = efi_systab->BootServices;
         void *retaddr = __builtin_return_address ( 0 );
         EFI_MEMORY_DESCRIPTOR *desc;
         size_t remaining;
         EFI_STATUS efirc;
 
         DBGC ( colour, "GetMemoryMap ( %#llx, %p ) ",
                ( ( unsigned long long ) *memory_map_size ), memory_map );
         efirc = bs->GetMemoryMap ( memory_map_size, memory_map, map_key,
                                    descriptor_size, descriptor_version );
         DBGC ( colour, "= %s ( %#llx, %#llx, %#llx, v%d",
                efi_status ( efirc ),
                ( ( unsigned long long ) *memory_map_size ),
                ( ( unsigned long long ) *map_key ),
                ( ( unsigned long long ) *descriptor_size ),
                *descriptor_version );
         if ( DBG_EXTRA && ( efirc == 0 ) ) {
                 DBGC2 ( colour, ",\n" );
                 for ( desc = memory_map, remaining = *memory_map_size ;
                       remaining >= *descriptor_size ;
                       desc = ( ( ( void * ) desc ) + *descriptor_size ),
                       remaining -= *descriptor_size ) {
                         DBGC2 ( colour, "%#016llx+%#08llx %#016llx "
                                 "%s\n", desc->PhysicalStart,
                                 ( desc->NumberOfPages * EFI_PAGE_SIZE ),
                                 desc->Attribute,
                                 efi_memory_type ( desc->Type ) );
                 }
         } else {
                 DBGC ( colour, " " );
         }
         DBGC ( colour, ") -> %p\n", retaddr );
         return efirc;
 }