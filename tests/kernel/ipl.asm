org 0x7c00
bits 16
	jmp entry

	BS_jmpBoot2	db	0x90
	BS_OEMName	db	"SAMPLE", 0x00, 0x00
	BPB_BytsPerSec	dw	0x0200		;BytesPerSector
	BPB_SecPerClus	db	0x01		;SectorPerCluster
	BPB_RsvdSecCnt	dw	0x0001		;ReservedSectors
	BPB_NumFATs	db	0x02		;TotalFATs
	BPB_RootEntCnt	dw	0x00e0		;MaxRootEntries
	BPB_TotSec16	dw	0x0b40		;TotalSectors
	BPB_Media	db	0xf0		;MediaDescriptor
	BPB_FATSz16	dw	0x0009		;SectorsPerFAT
	BPB_SecPerTrk	dw	0x0012		;SectorsPerTrack
	BPB_NumHeads	dw	0x0002		;NumHeads
	BPB_HiddSec	dd	0x00000000	;HiddenSector
	BPB_TotSec32	dd	0x00000000	;TotalSectors

	BS_DrvNum	db	0x00		;DriveNumber
	BS_Reserved1	db	0x00		;Reserved
	BS_BootSig	db	0x29		;BootSignature
	BS_VolID	dd	0x20210501	;VolumeSerialNumber
	BS_VolLab	db	"SampleKernel   "	;VolumeLabel
	BS_FilSysType	db	"FAT12   "	;FileSystemType

entry:
	mov sp, 0x7c00
	jmp next

times 0x1fe-($-$$) db 0
	dw 0xaa55

next: