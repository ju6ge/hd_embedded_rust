#[doc = "Reader of register MCAN_FBTP"]
pub type R = crate::R<u32, super::MCAN_FBTP>;
#[doc = "Writer for register MCAN_FBTP"]
pub type W = crate::W<u32, super::MCAN_FBTP>;
#[doc = "Register MCAN_FBTP `reset()`'s with value 0"]
impl crate::ResetValue for super::MCAN_FBTP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSJW`"]
pub type FSJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSJW`"]
pub struct FSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> FSJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FTSEG2`"]
pub type FTSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FTSEG2`"]
pub struct FTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `FTSEG1`"]
pub type FTSEG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FTSEG1`"]
pub struct FTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FBRP`"]
pub type FBRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FBRP`"]
pub struct FBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> FBRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Transceiver Delay Compensation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDC_A {
    #[doc = "0: Transceiver Delay Compensation disabled."]
    DISABLED,
    #[doc = "1: Transceiver Delay Compensation enabled."]
    ENABLED,
}
impl From<TDC_A> for bool {
    #[inline(always)]
    fn from(variant: TDC_A) -> Self {
        match variant {
            TDC_A::DISABLED => false,
            TDC_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TDC`"]
pub type TDC_R = crate::R<bool, TDC_A>;
impl TDC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDC_A {
        match self.bits {
            false => TDC_A::DISABLED,
            true => TDC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDC_A::ENABLED
    }
}
#[doc = "Write proxy for field `TDC`"]
pub struct TDC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transceiver Delay Compensation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDC_A::DISABLED)
    }
    #[doc = "Transceiver Delay Compensation enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDC_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `TDCO`"]
pub type TDCO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDCO`"]
pub struct TDCO_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Fast (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn fsjw(&self) -> FSJW_R {
        FSJW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Fast Time Segment After Sample Point"]
    #[inline(always)]
    pub fn ftseg2(&self) -> FTSEG2_R {
        FTSEG2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Fast Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn ftseg1(&self) -> FTSEG1_R {
        FTSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Fast Baud Rate Prescaler"]
    #[inline(always)]
    pub fn fbrp(&self) -> FBRP_R {
        FBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Fast (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn fsjw(&mut self) -> FSJW_W {
        FSJW_W { w: self }
    }
    #[doc = "Bits 4:6 - Fast Time Segment After Sample Point"]
    #[inline(always)]
    pub fn ftseg2(&mut self) -> FTSEG2_W {
        FTSEG2_W { w: self }
    }
    #[doc = "Bits 8:11 - Fast Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn ftseg1(&mut self) -> FTSEG1_W {
        FTSEG1_W { w: self }
    }
    #[doc = "Bits 16:20 - Fast Baud Rate Prescaler"]
    #[inline(always)]
    pub fn fbrp(&mut self) -> FBRP_W {
        FBRP_W { w: self }
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W {
        TDC_W { w: self }
    }
    #[doc = "Bits 24:28 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&mut self) -> TDCO_W {
        TDCO_W { w: self }
    }
}
