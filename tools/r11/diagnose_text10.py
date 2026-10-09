"""First named column in normalized release statistics synchronization traces."""
import sys
from pathlib import Path


def statistics(java, rust):
    left=Path(java).read_text().splitlines();right=Path(rust).read_text().splitlines()
    fields=['receiveReady','unsentStats','currentStats','unsentFile','oldUnsentFile','temporaryUnsentFile','receivedFile','achievements']
    for snapshot,(j,r) in enumerate(zip(left,right)):
        if j==r:continue
        for column,(a,b) in enumerate(zip(j.split('|'),r.split('|'))):
            if a==b:continue
            field=fields[column]
            if field=='achievements':
                ja=a.rstrip(';').split(';');ra=b.rstrip(';').split(';')
                for index,(av,bv) in enumerate(zip(ja,ra)):
                    if av!=bv:
                        for component,(ac,bc) in enumerate(zip(av.split(','),bv.split(','))):
                            if ac!=bc:return f'snapshot {snapshot}.achievement[{index}].'+['unlocked','canUnlock'][component]+f': Java {ac}, Rust {bc}'
                return f'snapshot {snapshot}.achievement[{min(len(ja),len(ra))}].present: Java {len(ja)>len(ra)}, Rust {len(ra)>len(ja)} (registry length Java {len(ja)}, Rust {len(ra)})'
            return f'snapshot {snapshot}.{field}: Java {a!r}, Rust {b!r}'
    if len(left)!=len(right):return f'snapshot count: Java {len(left)}, Rust {len(right)}'
    return 'EXACT'


if __name__=='__main__':print('FIRST DIFFERENCE: '+statistics(*sys.argv[1:]))
