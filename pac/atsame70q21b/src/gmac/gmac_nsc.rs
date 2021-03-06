#[doc = "Reader of register GMAC_NSC"]
pub type R = crate::R<u32, super::GMAC_NSC>;
#[doc = "Writer for register GMAC_NSC"]
pub type W = crate::W<u32, super::GMAC_NSC>;
#[doc = "Register GMAC_NSC `reset()`'s with value 0"]
impl crate::ResetValue for super::GMAC_NSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NANOSEC`"]
pub type NANOSEC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NANOSEC`"]
pub struct NANOSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NANOSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - 1588 Timer Nanosecond Comparison Value"]
    #[inline(always)]
    pub fn nanosec(&self) -> NANOSEC_R {
        NANOSEC_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - 1588 Timer Nanosecond Comparison Value"]
    #[inline(always)]
    pub fn nanosec(&mut self) -> NANOSEC_W {
        NANOSEC_W { w: self }
    }
}
