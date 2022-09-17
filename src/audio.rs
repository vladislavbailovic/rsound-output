use crate::{Buffer, OutputRenderer};
use std::io::Write;

pub struct WaveRenderer {
    pcm: PcmRenderer,
}

impl WaveRenderer {
    pub fn new(raw: &[f64], sample_rate: i32) -> Self {
        Self {
            pcm: PcmRenderer::new(raw, sample_rate),
        }
    }
}

impl Buffer for WaveRenderer {
    fn get_buffer(&self) -> &[u8] {
        &self.pcm.buffer
    }
}

impl OutputRenderer for WaveRenderer {
    /// As per: http://soundfile.sapp.org/doc/WaveFormat/
    fn get_header(&self) -> Option<Vec<u8>> {
        let buflen = self.pcm.buffer.len();
        let fsize = buflen + 44;

        let bits_per_sample = 64;
        let byte_rate = self.pcm.sample_rate * bits_per_sample / 8;
        let block_align = bits_per_sample / 8;
        let mut buf = Vec::new();

        let _ = buf.write(b"RIFF").ok()?;

        assert_eq!(buf.len(), 4, "4|4|ChunkSize|44 + SubChunk2Size");
        let _ = buf.write(&(fsize as u32).to_le_bytes()).ok()?;

        assert_eq!(buf.len(), 8, "8|4|Format|Contains the letters 'WAVE'");
        let _ = buf.write(b"WAVE").ok()?;
        let _ = buf.write(b"fmt ").ok()?;

        assert_eq!(buf.len(), 16, "16|4|Subchunk1Size|16 for PCM");
        let _ = buf.write(&16_u32.to_le_bytes()).ok()?;

        assert_eq!(
            buf.len(),
            20,
            "20|2|AudioFormat|PCM = 1 (i.e. Linear quantization)"
        );
        let _ = buf.write(&1_u16.to_le_bytes()).ok()?;

        assert_eq!(buf.len(), 22, "22|2|NumChannels|Mono = 1, Stereo = 2, etc.");
        let _ = buf.write(&1_u16.to_le_bytes()).ok()?;

        assert_eq!(buf.len(), 24, "24|4|SampleRate|8000, 44100, etc.");
        let _ = buf
            .write(&(self.pcm.sample_rate as u32).to_le_bytes())
            .ok()?;

        assert_eq!(
            buf.len(),
            28,
            "28|4|ByteRate|== SampleRate * NumChannels * BitsPerSample/8"
        );
        let _ = buf.write(&(byte_rate as u32).to_le_bytes()).ok()?;

        assert_eq!(
            buf.len(),
            32,
            "32|2|BlockAlign|== NumChannels * BitsPerSample/8"
        );
        let _ = buf.write(&(block_align as u16).to_le_bytes()).ok()?;

        assert_eq!(
            buf.len(),
            34,
            "34|2|BitsPerSample|8 bits = 8, 16 bits = 16, etc."
        );
        let _ = buf.write(&(bits_per_sample as u16).to_le_bytes()).ok()?;

        assert_eq!(
            buf.len(),
            36,
            "36|4|Subchunk2ID|Contains the letters 'data'"
        );
        let _ = buf.write(b"data").ok()?;

        assert_eq!(
            buf.len(),
            40,
            "40|4|Subchunk2Size|== NumSamples * NumChannels * BitsPerSample/8"
        );
        let _ = buf.write(&(buflen as u32).to_le_bytes()).ok()?;

        assert_eq!(buf.len(), 44, "total header length");
        Some(buf)
    }

    fn get_footer(&self) -> Option<Vec<u8>> {
        None
    }
}

// -------------------------------

pub struct PcmRenderer {
    buffer: Vec<u8>,
    sample_rate: i32,
}

impl PcmRenderer {
    pub fn new(raw: &[f64], sample_rate: i32) -> Self {
        let buffer = raw
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .collect::<Vec<u8>>();
        Self {
            buffer,
            sample_rate,
        }
    }
}

impl Buffer for PcmRenderer {
    fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl OutputRenderer for PcmRenderer {
    fn get_header(&self) -> Option<Vec<u8>> {
        None
    }
    fn get_footer(&self) -> Option<Vec<u8>> {
        None
    }
}
