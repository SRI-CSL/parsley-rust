#!/usr/bin/python
import subprocess, sys, os

def num_conversations(pcap):
    args  = ["tshark", "-q", "-2", "-R", "rtps", "-r", pcap, "-z", "conv,udp"]
    comp  = subprocess.run(args, capture_output=True)
    convs = comp.stdout.decode(encoding="utf-8").split("\n")
    return len(convs) - (5) - (2)

def get_conversation_packets(pcap, n):
    args = ["tshark", "-q", "-2", "-R", "rtps", "-r", pcap, "-z", ("follow,udp,raw,%d" % n)]
    comp = subprocess.run(args, capture_output=True)
    conv = comp.stdout.decode(encoding="utf-8").split("\n")
    return conv[6 : (len(conv) - 2)]

def save_conv(outdir, pcap, nconv, conv):
    pcap = os.path.split(pcap)[1]
    for npack, p in enumerate(conv):
        f = open("%s/%s-conv-%d-packet-%d.dat" % (outdir, pcap, nconv, npack) , "wb")
        f.write(p)
        f.close()

def process_pcap(pcap, outdir):
    n = num_conversations(pcap)
    for i in range(0, n):
        sconv = get_conversation_packets(pcap, i)
        bconv = [ bytes.fromhex(pack) for pack in sconv ]
        save_conv(outdir, pcap, i, bconv)

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: %s <pcap_file> <output_dir>" % sys.argv[0])
        sys.exit(1)
    process_pcap(sys.argv[1], sys.argv[2])
