using System;
using System.IO;

namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Special Big Endian stream implementation.
    /// https://github.com/dotnet/runtime/issues/26904
    /// </summary>
    public class BigEndianStream
    {
        private readonly Stream m_stream;

        public BigEndianStream(Stream stream)
        {
            m_stream = stream;
        }

        public bool HasRemaining()
        {
            return m_stream.Length - m_stream.Position > 0;
        }

        public long Position
        {
            get => m_stream.Position;
            set => m_stream.Position = value;
        }

        public void WriteBytes(byte[] value)
        {
            m_stream.Write(value, 0, value.Length);
        }

        public void WriteByte(byte value)
        {
            m_stream.WriteByte(value);
        }

        public void WriteUShort(ushort value)
        {
            m_stream.WriteByte((byte)(value >> 8));
            m_stream.WriteByte((byte)value);
        }

        public void WriteUInt(uint value)
        {
            m_stream.WriteByte((byte)(value >> 24));
            m_stream.WriteByte((byte)(value >> 16));
            m_stream.WriteByte((byte)(value >> 8));
            m_stream.WriteByte((byte)value);
        }

        public void WriteULong(ulong value)
        {
            WriteUInt((uint)(value >> 32));
            WriteUInt((uint)value);
        }

        public void WriteSByte(sbyte value)
        {
            m_stream.WriteByte((byte)value);
        }

        public void WriteShort(short value)
        {
            WriteUShort((ushort)value);
        }

        public void WriteInt(int value)
        {
            WriteUInt((uint)value);
        }

        public void WriteFloat(float value)
        {
            unsafe
            {
                WriteInt(*(int*)&value);
            }
        }

        public void WriteLong(long value)
        {
            WriteULong((ulong)value);
        }

        public void WriteDouble(double value)
        {
            WriteLong(BitConverter.DoubleToInt64Bits(value));
        }

        public byte[] ReadBytes(int length)
        {
            CheckRemaining(length);
            byte[] result = new byte[length];
            m_stream.Read(result, 0, length);
            return result;
        }

        public byte ReadByte()
        {
            CheckRemaining(1);
            return Convert.ToByte(m_stream.ReadByte());
        }

        public ushort ReadUShort()
        {
            CheckRemaining(2);
            return (ushort)(m_stream.ReadByte() << 8 | m_stream.ReadByte());
        }

        public uint ReadUInt()
        {
            CheckRemaining(4);
            return (uint)(
                m_stream.ReadByte() << 24
                | m_stream.ReadByte() << 16
                | m_stream.ReadByte() << 8
                | m_stream.ReadByte()
            );
        }

        public ulong ReadULong()
        {
            return (ulong)ReadUInt() << 32 | ReadUInt();
        }

        public sbyte ReadSByte()
        {
            return (sbyte)ReadByte();
        }

        public short ReadShort()
        {
            return (short)ReadUShort();
        }

        public int ReadInt()
        {
            return (int)ReadUInt();
        }

        public float ReadFloat()
        {
            unsafe
            {
                int value = ReadInt();
                return *(float*)&value;
            }
        }

        public long ReadLong()
        {
            return (long)ReadULong();
        }

        public double ReadDouble()
        {
            return BitConverter.Int64BitsToDouble(ReadLong());
        }

        private void CheckRemaining(int length)
        {
            if (m_stream.Length - m_stream.Position < length)
            {
                throw new StreamUnderflowException();
            }
        }
    }
}