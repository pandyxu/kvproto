// Code generated by protoc-gen-go.
// source: raft_serverpb.proto
// DO NOT EDIT!

/*
Package raft_serverpb is a generated protocol buffer package.

It is generated from these files:
	raft_serverpb.proto

It has these top-level messages:
	RaftMessage
	RaftTruncatedState
	KeyValue
	RaftSnapshotData
	StoreIdent
*/
package raft_serverpb

import proto "github.com/golang/protobuf/proto"
import fmt "fmt"
import math "math"
import raftpb "github.com/pingcap/kvproto/pkg/raftpb"
import metapb "github.com/pingcap/kvproto/pkg/metapb"

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
const _ = proto.ProtoPackageIsVersion1

type RaftMessage struct {
	RegionId         *uint64             `protobuf:"varint,1,opt,name=region_id" json:"region_id,omitempty"`
	FromPeer         *metapb.Peer        `protobuf:"bytes,2,opt,name=from_peer" json:"from_peer,omitempty"`
	ToPeer           *metapb.Peer        `protobuf:"bytes,3,opt,name=to_peer" json:"to_peer,omitempty"`
	Message          *raftpb.Message     `protobuf:"bytes,4,opt,name=message" json:"message,omitempty"`
	RegionEpoch      *metapb.RegionEpoch `protobuf:"bytes,5,opt,name=region_epoch" json:"region_epoch,omitempty"`
	XXX_unrecognized []byte              `json:"-"`
}

func (m *RaftMessage) Reset()                    { *m = RaftMessage{} }
func (m *RaftMessage) String() string            { return proto.CompactTextString(m) }
func (*RaftMessage) ProtoMessage()               {}
func (*RaftMessage) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *RaftMessage) GetRegionId() uint64 {
	if m != nil && m.RegionId != nil {
		return *m.RegionId
	}
	return 0
}

func (m *RaftMessage) GetFromPeer() *metapb.Peer {
	if m != nil {
		return m.FromPeer
	}
	return nil
}

func (m *RaftMessage) GetToPeer() *metapb.Peer {
	if m != nil {
		return m.ToPeer
	}
	return nil
}

func (m *RaftMessage) GetMessage() *raftpb.Message {
	if m != nil {
		return m.Message
	}
	return nil
}

func (m *RaftMessage) GetRegionEpoch() *metapb.RegionEpoch {
	if m != nil {
		return m.RegionEpoch
	}
	return nil
}

type RaftTruncatedState struct {
	Index            *uint64 `protobuf:"varint,1,opt,name=index" json:"index,omitempty"`
	Term             *uint64 `protobuf:"varint,2,opt,name=term" json:"term,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *RaftTruncatedState) Reset()                    { *m = RaftTruncatedState{} }
func (m *RaftTruncatedState) String() string            { return proto.CompactTextString(m) }
func (*RaftTruncatedState) ProtoMessage()               {}
func (*RaftTruncatedState) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *RaftTruncatedState) GetIndex() uint64 {
	if m != nil && m.Index != nil {
		return *m.Index
	}
	return 0
}

func (m *RaftTruncatedState) GetTerm() uint64 {
	if m != nil && m.Term != nil {
		return *m.Term
	}
	return 0
}

type KeyValue struct {
	Key              []byte `protobuf:"bytes,1,opt,name=key" json:"key,omitempty"`
	Value            []byte `protobuf:"bytes,2,opt,name=value" json:"value,omitempty"`
	XXX_unrecognized []byte `json:"-"`
}

func (m *KeyValue) Reset()                    { *m = KeyValue{} }
func (m *KeyValue) String() string            { return proto.CompactTextString(m) }
func (*KeyValue) ProtoMessage()               {}
func (*KeyValue) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{2} }

func (m *KeyValue) GetKey() []byte {
	if m != nil {
		return m.Key
	}
	return nil
}

func (m *KeyValue) GetValue() []byte {
	if m != nil {
		return m.Value
	}
	return nil
}

type RaftSnapshotData struct {
	Region           *metapb.Region `protobuf:"bytes,1,opt,name=region" json:"region,omitempty"`
	FileSize         *uint64        `protobuf:"varint,2,opt,name=file_size" json:"file_size,omitempty"`
	Data             []*KeyValue    `protobuf:"bytes,3,rep,name=data" json:"data,omitempty"`
	XXX_unrecognized []byte         `json:"-"`
}

func (m *RaftSnapshotData) Reset()                    { *m = RaftSnapshotData{} }
func (m *RaftSnapshotData) String() string            { return proto.CompactTextString(m) }
func (*RaftSnapshotData) ProtoMessage()               {}
func (*RaftSnapshotData) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{3} }

func (m *RaftSnapshotData) GetRegion() *metapb.Region {
	if m != nil {
		return m.Region
	}
	return nil
}

func (m *RaftSnapshotData) GetFileSize() uint64 {
	if m != nil && m.FileSize != nil {
		return *m.FileSize
	}
	return 0
}

func (m *RaftSnapshotData) GetData() []*KeyValue {
	if m != nil {
		return m.Data
	}
	return nil
}

type StoreIdent struct {
	ClusterId        *uint64 `protobuf:"varint,1,opt,name=cluster_id" json:"cluster_id,omitempty"`
	StoreId          *uint64 `protobuf:"varint,2,opt,name=store_id" json:"store_id,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *StoreIdent) Reset()                    { *m = StoreIdent{} }
func (m *StoreIdent) String() string            { return proto.CompactTextString(m) }
func (*StoreIdent) ProtoMessage()               {}
func (*StoreIdent) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{4} }

func (m *StoreIdent) GetClusterId() uint64 {
	if m != nil && m.ClusterId != nil {
		return *m.ClusterId
	}
	return 0
}

func (m *StoreIdent) GetStoreId() uint64 {
	if m != nil && m.StoreId != nil {
		return *m.StoreId
	}
	return 0
}

func init() {
	proto.RegisterType((*RaftMessage)(nil), "raft_serverpb.RaftMessage")
	proto.RegisterType((*RaftTruncatedState)(nil), "raft_serverpb.RaftTruncatedState")
	proto.RegisterType((*KeyValue)(nil), "raft_serverpb.KeyValue")
	proto.RegisterType((*RaftSnapshotData)(nil), "raft_serverpb.RaftSnapshotData")
	proto.RegisterType((*StoreIdent)(nil), "raft_serverpb.StoreIdent")
}

var fileDescriptor0 = []byte{
	// 320 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0x6c, 0x91, 0x51, 0x4b, 0xc3, 0x30,
	0x14, 0x85, 0x99, 0xcb, 0xdc, 0x76, 0x9b, 0xe9, 0xcc, 0x1e, 0x2c, 0x03, 0x75, 0x14, 0x14, 0x7d,
	0x29, 0xb8, 0xdf, 0xa0, 0x0f, 0x22, 0x82, 0x6c, 0xe2, 0x6b, 0x89, 0xeb, 0xed, 0x56, 0x6c, 0x9b,
	0x92, 0xa4, 0xc3, 0xf9, 0x9f, 0xfc, 0x8f, 0x26, 0x69, 0x8b, 0x14, 0x7c, 0x29, 0xe4, 0x9c, 0x7b,
	0xce, 0xfd, 0xb8, 0x85, 0x99, 0xe4, 0x89, 0x8e, 0x14, 0xca, 0x3d, 0xca, 0xf2, 0x23, 0x2c, 0xa5,
	0xd0, 0x82, 0x4d, 0x3a, 0xe2, 0x9c, 0xda, 0x67, 0x6b, 0xce, 0x69, 0x8e, 0x9a, 0xb7, 0xaf, 0xe0,
	0xa7, 0x07, 0xde, 0xca, 0xd8, 0x2f, 0xa8, 0x14, 0xdf, 0x22, 0x3b, 0x83, 0xb1, 0xc4, 0x6d, 0x2a,
	0x8a, 0x28, 0x8d, 0xfd, 0xde, 0xa2, 0x77, 0x4b, 0xd8, 0x15, 0x8c, 0x13, 0x29, 0xf2, 0xa8, 0x44,
	0x94, 0xfe, 0x91, 0x91, 0xbc, 0x25, 0x0d, 0x9b, 0x92, 0x57, 0xa3, 0xb1, 0x0b, 0x18, 0x6a, 0x51,
	0xdb, 0xfd, 0x7f, 0xec, 0x05, 0x0c, 0xf3, 0xba, 0xdd, 0x27, 0xce, 0x3e, 0x0d, 0x1b, 0xa0, 0x76,
	0xe9, 0x1d, 0xd0, 0x66, 0x29, 0x96, 0x62, 0xb3, 0xf3, 0x07, 0x6e, 0x6c, 0xd6, 0xb6, 0xac, 0x9c,
	0xf7, 0x68, 0xad, 0xe0, 0x1e, 0x98, 0xc5, 0x7d, 0x93, 0x55, 0xb1, 0xe1, 0x1a, 0xe3, 0xb5, 0x36,
	0x5f, 0x36, 0x81, 0x41, 0x5a, 0xc4, 0xf8, 0xd5, 0x10, 0x53, 0x20, 0x1a, 0x65, 0xee, 0x60, 0x49,
	0x70, 0x03, 0xa3, 0x67, 0x3c, 0xbc, 0xf3, 0xac, 0x42, 0xe6, 0x41, 0xff, 0x13, 0x0f, 0x6e, 0x8c,
	0xda, 0xd4, 0xde, 0xaa, 0x6e, 0x8e, 0x06, 0x19, 0x4c, 0x6d, 0xf5, 0xba, 0xe0, 0xa5, 0xda, 0x09,
	0xfd, 0xc0, 0x35, 0x67, 0x97, 0x70, 0x5c, 0x93, 0xb9, 0x88, 0xb7, 0x3c, 0xe9, 0x32, 0xd9, 0x73,
	0x25, 0x69, 0x86, 0x91, 0x4a, 0xbf, 0xeb, 0x1a, 0xc2, 0xae, 0x81, 0xc4, 0x26, 0x6a, 0x4e, 0xd1,
	0x37, 0x81, 0xf3, 0xb0, 0xfb, 0x83, 0x5a, 0x92, 0x60, 0x09, 0xb0, 0xd6, 0x42, 0xe2, 0x53, 0x8c,
	0x85, 0x66, 0x0c, 0x60, 0x93, 0x55, 0xca, 0x50, 0xff, 0xdd, 0x7d, 0x0a, 0x23, 0x65, 0x27, 0xac,
	0xe2, 0xaa, 0x7f, 0x03, 0x00, 0x00, 0xff, 0xff, 0x10, 0x85, 0x2e, 0x62, 0xed, 0x01, 0x00, 0x00,
}
