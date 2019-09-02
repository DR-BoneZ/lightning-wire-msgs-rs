package main

import (
	"bytes"
	"log"
	"time"

	"github.com/btcsuite/btcd/chaincfg/chainhash"
	"github.com/lightningnetwork/lnd/lnwire"
	"github.com/lightningnetwork/lnd/watchtower/wtwire"
)

func main() {
	features := lnwire.NewRawFeatureVector(lnwire.DataLossProtectRequired, lnwire.GossipQueriesRequired, lnwire.InitialRoutingSync)
	hash, err := chainhash.NewHash(make([]byte, 32))
	if err != nil {
		log.Panicf("%s", err)
	}
	msg := wtwire.NewInitMessage(features, *hash)

	var expected bytes.Buffer
	msg.Encode(&expected, 0)

	var buf bytes.Buffer

	start := time.Now()
	for i := 0; i < 1000000; i++ {
		buf.Reset()
		msg.Encode(&buf, 0)
		if !bytes.Equal(buf.Bytes(), expected.Bytes()) {
			log.Panicf("Failed assertion!")
		}
		msg.Decode(&buf, 0)
	}
	elapsed := time.Since(start)
	log.Printf("%s", elapsed)

	return
}
