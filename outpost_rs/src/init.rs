use crate::defines::U32Ptr;
use crate::global::AppContext;
use crate::mem_1000::{mem_op_1000_179c, mem_op_1000_1902, mem_op_1000_1b68};
use crate::pass::pass_1000::{pass1_1000_1fea, pass1_1000_4d0c};
use crate::string::string_1008::str_op_1008_60e8;
use crate::struct_ops::struct_1008::struct_op_1008_0000;
use crate::sys_api::{dos3_call_op_1000_435c, win_msg_op_1008_9498};
use crate::util::{make_u16_ptr, make_u8_ptr, read_string_from_addr, CONCAT12, CONCAT13, CONCAT22};

pub unsafe fn init_1000_23be(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: &mut String,
    param_4: u16,
    param_5: u16,
    param_6: &mut i16,
) {
    let mut loaded_str = read_string_from_addr(CONCAT22(
        ctx.PTR_LOOP_1050_5f50 as u16,
        ctx.PTR_LOOP_1050_5f4e as u16,
    ));
    init_op_1008_54aa(
        make_u8_ptr(ctx.PTR_LOOP_1050_5f52),
        &mut loaded_str,
        make_u8_ptr(ctx.PTR_LOOP_1050_5f4a),
        make_u8_ptr(ctx.PTR_LOOP_1050_5f4c),
        ctx.data_seg,
        param_1,
        param_2,
        param_3,
        param_4,
        param_5,
        param_6,
    );
    return;
}

pub fn init_globals_1020_96d4(ctx: &mut AppContext) {
    let pu_var1: U32Ptr;
    let i_var2: i16;
    let pu_var3: u16;

    ctx.PTR_LOOP_1050_4514 = 0x0;
    ctx.PTR_LOOP_1050_451a = 0x0;
    ctx.PTR_LOOP_1050_4520 = 0x4430;
    ctx.PTR_LOOP_1050_4522 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4526 = 0x4430;
    ctx.PTR_LOOP_1050_4528 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4524 = ctx.PTR_LOOP_1050_4434;
    ctx.PTR_LOOP_1050_452a = ctx.PTR_LOOP_1050_4434;
    ctx.PTR_LOOP_1050_452c = 0x4430;
    ctx.PTR_LOOP_1050_452e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4530 = ctx.PTR_LOOP_1050_4434;
    ctx.PTR_LOOP_1050_4532 = 0x4430;
    ctx.PTR_LOOP_1050_4534 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4536 = ctx.PTR_LOOP_1050_4434;
    ctx.PTR_LOOP_1050_4538 = 0x0;
    ctx.PTR_LOOP_1050_453e = 0x0;
    ctx.PTR_LOOP_1050_4544 = 0x4436;
    ctx.PTR_LOOP_1050_4546 = ctx.data_seg;
    ctx.PTR_LOOP_1050_454a = 0x4436;
    ctx.PTR_LOOP_1050_454c = ctx.data_seg;
    ctx.PTR_LOOP_1050_4548 = ctx.PTR_LOOP_1050_443a;
    ctx.PTR_LOOP_1050_454e = ctx.PTR_LOOP_1050_443a;
    ctx.PTR_LOOP_1050_4550 = 0x4436;
    ctx.PTR_LOOP_1050_4552 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4554 = ctx.PTR_LOOP_1050_443a;
    ctx.PTR_LOOP_1050_4512 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_455a = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_4556 = 0x4454;
    ctx.PTR_LOOP_1050_4558 = ctx.data_seg;
    ctx.PTR_LOOP_1050_455c = 0x4454;
    ctx.PTR_LOOP_1050_455e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4560 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_4562 = 0x4454;
    ctx.PTR_LOOP_1050_4564 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4566 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_456a = 0x0;
    ctx.PTR_LOOP_1050_4568 = 0x0;
    ctx.PTR_LOOP_1050_456e = 0x443c;
    ctx.PTR_LOOP_1050_4570 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4574 = 0x443c;
    ctx.PTR_LOOP_1050_4576 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4572 = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_4578 = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_457a = 0x443c;
    ctx.PTR_LOOP_1050_457c = ctx.data_seg;
    ctx.PTR_LOOP_1050_457e = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_4580 = 0x443c;
    ctx.PTR_LOOP_1050_4582 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4584 = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_4586 = 0x443c;
    ctx.PTR_LOOP_1050_4588 = ctx.data_seg;
    ctx.PTR_LOOP_1050_458a = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_458c = 0x443c;
    ctx.PTR_LOOP_1050_458e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4590 = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_4592 = 0x4454;
    ctx.PTR_LOOP_1050_4594 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4596 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_4598 = 0x4454;
    ctx.PTR_LOOP_1050_459a = ctx.data_seg;
    ctx.PTR_LOOP_1050_459c = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_459e = 0x4436;
    ctx.PTR_LOOP_1050_45a0 = ctx.data_seg;
    ctx.PTR_LOOP_1050_45a2 = ctx.PTR_LOOP_1050_443a;
    ctx.PTR_LOOP_1050_45a4 = 0x4436;
    ctx.PTR_LOOP_1050_45a6 = ctx.data_seg;
    ctx.PTR_LOOP_1050_45a8 = ctx.PTR_LOOP_1050_443a;
    ctx.PTR_LOOP_1050_45aa = 0x0;
    ctx.PTR_LOOP_1050_45b0 = 0x0;
    ctx.PTR_LOOP_1050_45b6 = 0x0;
    ctx.PTR_LOOP_1050_45bc = 0x443c;
    ctx.PTR_LOOP_1050_45be = ctx.data_seg;
    ctx.PTR_LOOP_1050_45c0 = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_45c2 = 0x443c;
    ctx.PTR_LOOP_1050_45c4 = ctx.data_seg;
    ctx.PTR_LOOP_1050_45c6 = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_45c8 = 0x0;
    ctx.PTR_LOOP_1050_45ce = 0x0;
    ctx.PTR_LOOP_1050_45d4 = 0x0;
    ctx.PTR_LOOP_1050_45da = 0x0;
    ctx.PTR_LOOP_1050_45e0 = 0x443c;
    ctx.PTR_LOOP_1050_45e2 = ctx.data_seg;
    ctx.PTR_LOOP_1050_45e4 = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_45e6 = 0x443c;
    ctx.PTR_LOOP_1050_45e8 = ctx.data_seg;
    ctx.PTR_LOOP_1050_45ea = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_45ec = 0x0;
    ctx.PTR_LOOP_1050_45f2 = 0x0;
    ctx.PTR_LOOP_1050_45f8 = 0x0;
    ctx.PTR_LOOP_1050_45fe = 0x443c;
    ctx.PTR_LOOP_1050_4600 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4602 = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_4604 = 0x443c;
    ctx.PTR_LOOP_1050_4606 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4608 = ctx.DAT_1050_4446;
    ctx.PTR_LOOP_1050_460a = 0x0;
    ctx.PTR_LOOP_1050_4610 = 0x0;
    ctx.PTR_LOOP_1050_451e = 0xffff;
    ctx.PTR_LOOP_1050_45ae = 0xffff;
    ctx.PTR_LOOP_1050_45b4 = 0xffff;
    ctx.PTR_LOOP_1050_45ba = 0xffff;
    ctx.PTR_LOOP_1050_45cc = 0xffff;
    ctx.PTR_LOOP_1050_45d2 = 0xffff;
    ctx.PTR_LOOP_1050_45f6 = 0xffff;
    ctx.PTR_LOOP_1050_45fc = 0xffff;
    ctx.PTR_LOOP_1050_460e = 0xffff;
    ctx.PTR_LOOP_1050_4614 = 0xffff;
    ctx.PTR_LOOP_1050_4616 = 0x0;
    ctx.PTR_LOOP_1050_461c = 0x0;
    ctx.PTR_LOOP_1050_4622 = 0x0;
    ctx.PTR_LOOP_1050_4628 = 0x0;
    ctx.PTR_LOOP_1050_462e = 0x0;
    ctx.PTR_LOOP_1050_4634 = 0x0;
    ctx.PTR_LOOP_1050_4518 = 0x0;
    ctx.PTR_LOOP_1050_453c = 0x0;
    ctx.PTR_LOOP_1050_4542 = 0x0;
    ctx.PTR_LOOP_1050_456c = 0x0;
    ctx.PTR_LOOP_1050_45d8 = 0x0;
    ctx.PTR_LOOP_1050_45de = 0x0;
    ctx.PTR_LOOP_1050_45f0 = 0x0;
    ctx.PTR_LOOP_1050_461a = 0x0;
    ctx.PTR_LOOP_1050_4620 = 0x0;
    ctx.PTR_LOOP_1050_4626 = 0x0;
    ctx.PTR_LOOP_1050_462c = 0x0;
    ctx.PTR_LOOP_1050_4632 = 0x0;
    ctx.PTR_LOOP_1050_4638 = 0x0;
    ctx.PTR_LOOP_1050_463a = 0x0;
    ctx.PTR_LOOP_1050_4640 = 0x0;
    ctx.PTR_LOOP_1050_4646 = 0x0;
    ctx.PTR_LOOP_1050_464c = 0x0;
    ctx.PTR_LOOP_1050_4652 = 0x0;
    ctx.PTR_LOOP_1050_4658 = 0x0;
    ctx.PTR_LOOP_1050_465e = 0x4448;
    ctx.PTR_LOOP_1050_4660 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4664 = 0x4448;
    ctx.PTR_LOOP_1050_4666 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4662 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_4668 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_466a = 0x4448;
    ctx.PTR_LOOP_1050_466c = ctx.data_seg;
    ctx.PTR_LOOP_1050_466e = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_4670 = 0x4454;
    ctx.PTR_LOOP_1050_4672 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4676 = 0x4454;
    ctx.PTR_LOOP_1050_4678 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4674 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_467a = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_467c = 0x4454;
    ctx.PTR_LOOP_1050_467e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4680 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_4682 = 0x4454;
    ctx.PTR_LOOP_1050_4684 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4686 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_4688 = 0x4454;
    ctx.PTR_LOOP_1050_468a = ctx.data_seg;
    ctx.PTR_LOOP_1050_468c = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_468e = 0x4448;
    ctx.PTR_LOOP_1050_4690 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4692 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_4694 = 0x4448;
    ctx.PTR_LOOP_1050_4696 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4698 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_469a = 0x4448;
    ctx.PTR_LOOP_1050_469c = ctx.data_seg;
    ctx.PTR_LOOP_1050_469e = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_46a0 = 0x4448;
    ctx.PTR_LOOP_1050_46a2 = ctx.data_seg;
    ctx.PTR_LOOP_1050_46a4 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_46a6 = 0x4454;
    ctx.PTR_LOOP_1050_46a8 = ctx.data_seg;
    ctx.PTR_LOOP_1050_46aa = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_46ac = 0x4454;
    ctx.PTR_LOOP_1050_46ae = ctx.data_seg;
    ctx.PTR_LOOP_1050_46b0 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_46b2 = 0x4454;
    ctx.PTR_LOOP_1050_46b4 = ctx.data_seg;
    ctx.PTR_LOOP_1050_46b6 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_46b8 = 0x4454;
    ctx.PTR_LOOP_1050_46ba = ctx.data_seg;
    ctx.PTR_LOOP_1050_46bc = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_46be = 0x4454;
    ctx.PTR_LOOP_1050_46c0 = ctx.data_seg;
    ctx.PTR_LOOP_1050_46c2 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_46c6 = 0x0;
    ctx.PTR_LOOP_1050_46c4 = 0x0;
    ctx.PTR_LOOP_1050_46cc = 0x0;
    ctx.PTR_LOOP_1050_46ca = 0x0;
    ctx.PTR_LOOP_1050_46d2 = 0x0;
    ctx.PTR_LOOP_1050_46d0 = 0x0;
    ctx.PTR_LOOP_1050_46d8 = 0x0;
    ctx.PTR_LOOP_1050_46d6 = 0x0;
    ctx.PTR_LOOP_1050_46de = 0x0;
    ctx.PTR_LOOP_1050_46dc = 0x0;
    ctx.PTR_LOOP_1050_46e2 = 0x4454;
    ctx.PTR_LOOP_1050_46e4 = ctx.data_seg;
    ctx.PTR_LOOP_1050_46e6 = ctx.DAT_1050_4462;
    ctx.PTR_LOOP_1050_46e8 = 0x4448;
    ctx.PTR_LOOP_1050_46ea = ctx.data_seg;
    ctx.PTR_LOOP_1050_46ec = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_46ee = 0x4448;
    ctx.PTR_LOOP_1050_46f0 = ctx.data_seg;
    ctx.PTR_LOOP_1050_46f2 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_46f4 = 0x0;
    ctx.PTR_LOOP_1050_46fa = 0x0;
    ctx.PTR_LOOP_1050_46f8 = 0xffff;
    ctx.PTR_LOOP_1050_46fe = 0xffff;
    ctx.PTR_LOOP_1050_4700 = 0x0;
    ctx.PTR_LOOP_1050_4706 = 0x0;
    ctx.PTR_LOOP_1050_470c = 0x4448;
    ctx.PTR_LOOP_1050_470e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4710 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_4712 = 0x4448;
    ctx.PTR_LOOP_1050_4714 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4716 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_4718 = 0x0;
    ctx.PTR_LOOP_1050_471e = 0x0;
    ctx.PTR_LOOP_1050_4724 = 0x0;
    ctx.PTR_LOOP_1050_472a = 0x0;
    ctx.PTR_LOOP_1050_4730 = 0x0;
    ctx.PTR_LOOP_1050_4736 = 0x0;
    ctx.PTR_LOOP_1050_473c = 0x0;
    ctx.PTR_LOOP_1050_4742 = 0x0;
    ctx.PTR_LOOP_1050_4748 = 0x0;
    ctx.PTR_LOOP_1050_474e = 0x0;
    ctx.PTR_LOOP_1050_4754 = 0x0;
    ctx.PTR_LOOP_1050_475a = 0x0;
    ctx.PTR_LOOP_1050_4760 = 0x0;
    ctx.PTR_LOOP_1050_463e = 0x0;
    ctx.PTR_LOOP_1050_4644 = 0x0;
    ctx.PTR_LOOP_1050_464a = 0x0;
    ctx.PTR_LOOP_1050_4650 = 0x0;
    ctx.PTR_LOOP_1050_4656 = 0x0;
    ctx.PTR_LOOP_1050_465c = 0x0;
    ctx.PTR_LOOP_1050_46c8 = 0x0;
    ctx.PTR_LOOP_1050_46ce = 0x0;
    ctx.PTR_LOOP_1050_46d4 = 0x0;
    ctx.PTR_LOOP_1050_46da = 0x0;
    ctx.PTR_LOOP_1050_46e0 = 0x0;
    ctx.PTR_LOOP_1050_4704 = 0x0;
    ctx.PTR_LOOP_1050_470a = 0x0;
    ctx.PTR_LOOP_1050_471c = 0x0;
    ctx.PTR_LOOP_1050_4722 = 0x0;
    ctx.PTR_LOOP_1050_4728 = 0x0;
    ctx.PTR_LOOP_1050_472e = 0x0;
    ctx.PTR_LOOP_1050_4734 = 0x0;
    ctx.PTR_LOOP_1050_473a = 0x0;
    ctx.PTR_LOOP_1050_4740 = 0x0;
    ctx.PTR_LOOP_1050_4746 = 0x0;
    ctx.PTR_LOOP_1050_474c = 0x0;
    ctx.PTR_LOOP_1050_4752 = 0x0;
    ctx.PTR_LOOP_1050_4758 = 0x0;
    ctx.PTR_LOOP_1050_475e = 0x0;
    ctx.PTR_LOOP_1050_4764 = 0x0;
    ctx.PTR_LOOP_1050_4766 = 0x0;
    ctx.PTR_LOOP_1050_476c = 0x0;
    ctx.PTR_LOOP_1050_4772 = 0x0;
    ctx.PTR_LOOP_1050_4778 = 0x0;
    ctx.PTR_LOOP_1050_477e = 0x0;
    ctx.PTR_LOOP_1050_4784 = 0x0;
    ctx.PTR_LOOP_1050_478a = 0x0;
    ctx.PTR_LOOP_1050_4790 = 0x0;
    ctx.PTR_LOOP_1050_4796 = 0x0;
    ctx.PTR_LOOP_1050_479c = 0x0;
    ctx.PTR_LOOP_1050_47a2 = 0x0;
    ctx.PTR_LOOP_1050_47a8 = 0x0;
    ctx.PTR_LOOP_1050_47ae = 0x0;
    ctx.PTR_LOOP_1050_47b4 = 0x0;
    ctx.PTR_LOOP_1050_476a = 0x0;
    ctx.PTR_LOOP_1050_4770 = 0x0;
    ctx.PTR_LOOP_1050_4776 = 0x0;
    ctx.PTR_LOOP_1050_477c = 0x0;
    ctx.PTR_LOOP_1050_4782 = 0x0;
    ctx.PTR_LOOP_1050_4788 = 0x0;
    ctx.PTR_LOOP_1050_478e = 0x0;
    ctx.PTR_LOOP_1050_4794 = 0x0;
    ctx.PTR_LOOP_1050_479a = 0x0;
    ctx.PTR_LOOP_1050_47a0 = 0x0;
    ctx.PTR_LOOP_1050_47a6 = 0x0;
    ctx.PTR_LOOP_1050_47ac = 0x0;
    ctx.PTR_LOOP_1050_47b2 = 0x0;
    ctx.PTR_LOOP_1050_47b8 = 0x0;
    pu_var3 = 0x47ba;
    // for (i_var2 = 0x1b; i_var2 != 0x0; i_var2 += -0x1) {
    //   pu_var1 = pu_var3;
    //   pu_var3 = pu_var3 + 0x1;
    //   *pu_var1 = 0x0;
    // }
    ctx.PTR_LOOP_1050_4850 = 0x0;
    ctx.PTR_LOOP_1050_4856 = 0x0;
    ctx.PTR_LOOP_1050_484e = ctx.PTR_LOOP_1050_4468;
    ctx.PTR_LOOP_1050_4860 = ctx.PTR_LOOP_1050_4468;
    ctx.PTR_LOOP_1050_485c = 0x4464;
    ctx.PTR_LOOP_1050_485e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4862 = 0x4464;
    ctx.PTR_LOOP_1050_4864 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4866 = ctx.PTR_LOOP_1050_4468;
    ctx.PTR_LOOP_1050_4868 = 0x4464;
    ctx.PTR_LOOP_1050_486a = ctx.data_seg;
    ctx.PTR_LOOP_1050_486c = ctx.PTR_LOOP_1050_4468;
    ctx.PTR_LOOP_1050_486e = 0x4464;
    ctx.PTR_LOOP_1050_4870 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4872 = ctx.PTR_LOOP_1050_4468;
    ctx.PTR_LOOP_1050_4874 = 0x0;
    ctx.PTR_LOOP_1050_487a = 0x0;
    ctx.PTR_LOOP_1050_4880 = 0x4436;
    ctx.PTR_LOOP_1050_4882 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4886 = 0x4436;
    ctx.PTR_LOOP_1050_4888 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4884 = ctx.PTR_LOOP_1050_443a;
    ctx.PTR_LOOP_1050_488a = ctx.PTR_LOOP_1050_443a;
    ctx.PTR_LOOP_1050_488c = 0x4436;
    ctx.PTR_LOOP_1050_488e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4890 = ctx.PTR_LOOP_1050_443a;
    ctx.PTR_LOOP_1050_4892 = 0x4482;
    ctx.PTR_LOOP_1050_4894 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4898 = 0x4482;
    ctx.PTR_LOOP_1050_489a = ctx.data_seg;
    ctx.PTR_LOOP_1050_4896 = ctx.PTR_LOOP_1050_4486;
    ctx.PTR_LOOP_1050_489c = ctx.PTR_LOOP_1050_4486;
    ctx.PTR_LOOP_1050_489e = 0x4482;
    ctx.PTR_LOOP_1050_48a0 = ctx.data_seg;
    ctx.PTR_LOOP_1050_48a2 = ctx.PTR_LOOP_1050_4486;
    ctx.PTR_LOOP_1050_48a6 = 0x0;
    ctx.PTR_LOOP_1050_48a4 = 0x0;
    ctx.PTR_LOOP_1050_48aa = 0x4488;
    ctx.PTR_LOOP_1050_48ac = ctx.data_seg;
    ctx.PTR_LOOP_1050_48b0 = 0x4488;
    ctx.PTR_LOOP_1050_48b2 = ctx.data_seg;
    ctx.PTR_LOOP_1050_48ae = ctx.PTR_LOOP_1050_448c;
    ctx.PTR_LOOP_1050_48b4 = ctx.PTR_LOOP_1050_448c;
    ctx.PTR_LOOP_1050_48b6 = 0x4488;
    ctx.PTR_LOOP_1050_48b8 = ctx.data_seg;
    ctx.PTR_LOOP_1050_48ba = ctx.PTR_LOOP_1050_448c;
    ctx.PTR_LOOP_1050_48bc = 0x446a;
    ctx.PTR_LOOP_1050_48be = ctx.data_seg;
    ctx.PTR_LOOP_1050_48c2 = 0x446a;
    ctx.PTR_LOOP_1050_48c4 = ctx.data_seg;
    ctx.PTR_LOOP_1050_48c0 = ctx.PTR_LOOP_1050_446e;
    ctx.PTR_LOOP_1050_48c6 = ctx.PTR_LOOP_1050_446e;
    ctx.PTR_LOOP_1050_48c8 = 0x446a;
    ctx.PTR_LOOP_1050_48ca = ctx.data_seg;
    ctx.PTR_LOOP_1050_48cc = ctx.PTR_LOOP_1050_446e;
    ctx.PTR_LOOP_1050_48ce = 0x447a;
    ctx.PTR_LOOP_1050_48d0 = ctx.data_seg;
    ctx.PTR_LOOP_1050_48d4 = 0x447a;
    ctx.PTR_LOOP_1050_48d6 = ctx.data_seg;
    ctx.PTR_LOOP_1050_48d2 = ctx.DAT_1050_4480;
    ctx.PTR_LOOP_1050_48d8 = ctx.DAT_1050_4480;
    ctx.PTR_LOOP_1050_48da = 0x4436;
    ctx.PTR_LOOP_1050_48dc = ctx.data_seg;
    ctx.PTR_LOOP_1050_48de = ctx.PTR_LOOP_1050_443a;
    ctx.PTR_LOOP_1050_48e0 = 0x4436;
    ctx.PTR_LOOP_1050_48e2 = ctx.data_seg;
    ctx.PTR_LOOP_1050_48e4 = ctx.PTR_LOOP_1050_443a;
    ctx.PTR_LOOP_1050_48e6 = 0x447a;
    ctx.PTR_LOOP_1050_48e8 = ctx.data_seg;
    ctx.PTR_LOOP_1050_48ea = ctx.DAT_1050_4480;
    ctx.PTR_LOOP_1050_48ec = 0x0;
    ctx.PTR_LOOP_1050_48f2 = 0x0;
    ctx.PTR_LOOP_1050_48f8 = 0x447a;
    ctx.PTR_LOOP_1050_48fa = ctx.data_seg;
    ctx.PTR_LOOP_1050_48fc = ctx.DAT_1050_4480;
    ctx.PTR_LOOP_1050_48fe = 0x447a;
    ctx.PTR_LOOP_1050_4900 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4902 = ctx.DAT_1050_4480;
    ctx.PTR_LOOP_1050_4904 = 0x0;
    ctx.PTR_LOOP_1050_490a = 0x0;
    ctx.PTR_LOOP_1050_485a = 0xffff;
    ctx.PTR_LOOP_1050_48f0 = 0xffff;
    ctx.PTR_LOOP_1050_48f6 = 0xffff;
    ctx.PTR_LOOP_1050_4908 = 0xffff;
    ctx.PTR_LOOP_1050_490e = 0xffff;
    ctx.PTR_LOOP_1050_4910 = 0x0;
    ctx.PTR_LOOP_1050_4916 = 0x0;
    ctx.PTR_LOOP_1050_4854 = 0x0;
    ctx.PTR_LOOP_1050_4878 = 0x0;
    ctx.PTR_LOOP_1050_487e = 0x0;
    ctx.PTR_LOOP_1050_48a8 = 0x0;
    ctx.PTR_LOOP_1050_4914 = 0x0;
    ctx.PTR_LOOP_1050_491a = 0x0;
    ctx.PTR_LOOP_1050_491c = 0x4488;
    ctx.PTR_LOOP_1050_491e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4920 = ctx.PTR_LOOP_1050_448c;
    ctx.PTR_LOOP_1050_4922 = 0x4488;
    ctx.PTR_LOOP_1050_4924 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4926 = ctx.PTR_LOOP_1050_448c;
    ctx.PTR_LOOP_1050_4928 = 0x0;
    ctx.PTR_LOOP_1050_492e = 0x0;
    ctx.PTR_LOOP_1050_4934 = 0x0;
    ctx.PTR_LOOP_1050_493a = 0x446a;
    ctx.PTR_LOOP_1050_493c = ctx.data_seg;
    ctx.PTR_LOOP_1050_4940 = 0x446a;
    ctx.PTR_LOOP_1050_4942 = ctx.data_seg;
    ctx.PTR_LOOP_1050_493e = ctx.PTR_LOOP_1050_446e;
    ctx.PTR_LOOP_1050_4944 = ctx.PTR_LOOP_1050_446e;
    ctx.PTR_LOOP_1050_4946 = 0x0;
    ctx.PTR_LOOP_1050_494c = 0x0;
    ctx.PTR_LOOP_1050_4952 = 0x0;
    ctx.PTR_LOOP_1050_4958 = 0x0;
    ctx.PTR_LOOP_1050_495e = 0x0;
    ctx.PTR_LOOP_1050_4964 = 0x0;
    ctx.PTR_LOOP_1050_496a = 0x0;
    ctx.PTR_LOOP_1050_4970 = 0x0;
    ctx.PTR_LOOP_1050_4976 = 0x0;
    ctx.PTR_LOOP_1050_497c = 0x0;
    ctx.PTR_LOOP_1050_4982 = 0x0;
    ctx.PTR_LOOP_1050_4988 = 0x0;
    ctx.PTR_LOOP_1050_498e = 0x0;
    ctx.PTR_LOOP_1050_4994 = 0x0;
    ctx.PTR_LOOP_1050_499a = 0x4448;
    ctx.PTR_LOOP_1050_499c = ctx.data_seg;
    ctx.PTR_LOOP_1050_49a0 = 0x4448;
    ctx.PTR_LOOP_1050_49a2 = ctx.data_seg;
    ctx.PTR_LOOP_1050_499e = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_49a4 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_49a6 = 0x4448;
    ctx.PTR_LOOP_1050_49a8 = ctx.data_seg;
    ctx.PTR_LOOP_1050_49aa = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_49ac = 0x4470;
    ctx.PTR_LOOP_1050_49ae = ctx.data_seg;
    ctx.PTR_LOOP_1050_49b2 = 0x4470;
    ctx.PTR_LOOP_1050_49b4 = ctx.data_seg;
    ctx.PTR_LOOP_1050_49b0 = ctx.PTR_DAT_1050_0004_1050_4478;
    ctx.PTR_LOOP_1050_49b6 = ctx.PTR_DAT_1050_0004_1050_4478;
    ctx.PTR_LOOP_1050_49b8 = 0x4470;
    ctx.PTR_LOOP_1050_49ba = ctx.data_seg;
    ctx.PTR_LOOP_1050_49bc = ctx.PTR_DAT_1050_0004_1050_4478;
    ctx.PTR_LOOP_1050_49be = 0x4470;
    ctx.PTR_LOOP_1050_49c0 = ctx.data_seg;
    ctx.PTR_LOOP_1050_49c2 = ctx.PTR_DAT_1050_0004_1050_4478;
    ctx.PTR_LOOP_1050_49c4 = 0x4470;
    ctx.PTR_LOOP_1050_49c6 = ctx.data_seg;
    ctx.PTR_LOOP_1050_49c8 = ctx.PTR_DAT_1050_0004_1050_4478;
    ctx.PTR_LOOP_1050_49ca = 0x4448;
    ctx.PTR_LOOP_1050_49cc = ctx.data_seg;
    ctx.PTR_LOOP_1050_49ce = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_49d0 = 0x4448;
    ctx.PTR_LOOP_1050_49d2 = ctx.data_seg;
    ctx.PTR_LOOP_1050_49d4 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_49d6 = 0x4448;
    ctx.PTR_LOOP_1050_49d8 = ctx.data_seg;
    ctx.PTR_LOOP_1050_49da = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_49dc = 0x4448;
    ctx.PTR_LOOP_1050_49de = ctx.data_seg;
    ctx.PTR_LOOP_1050_49e0 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_49e2 = 0x4482;
    ctx.PTR_LOOP_1050_49e4 = ctx.data_seg;
    ctx.PTR_LOOP_1050_49e8 = 0x4482;
    ctx.PTR_LOOP_1050_49ea = ctx.data_seg;
    ctx.PTR_LOOP_1050_49e6 = ctx.PTR_LOOP_1050_4486;
    ctx.PTR_LOOP_1050_49ec = ctx.PTR_LOOP_1050_4486;
    ctx.PTR_LOOP_1050_49ee = 0x4470;
    ctx.PTR_LOOP_1050_49f0 = ctx.data_seg;
    ctx.PTR_LOOP_1050_49f2 = ctx.PTR_DAT_1050_0004_1050_4478;
    ctx.PTR_LOOP_1050_49f4 = 0x4470;
    ctx.PTR_LOOP_1050_49f6 = ctx.data_seg;
    ctx.PTR_LOOP_1050_49f8 = ctx.PTR_DAT_1050_0004_1050_4478;
    ctx.PTR_LOOP_1050_49fa = 0x4470;
    ctx.PTR_LOOP_1050_49fc = ctx.data_seg;
    ctx.PTR_LOOP_1050_49fe = ctx.PTR_DAT_1050_0004_1050_4478;
    ctx.PTR_LOOP_1050_4a02 = 0x0;
    ctx.PTR_LOOP_1050_4a00 = 0x0;
    ctx.PTR_LOOP_1050_4a08 = 0x0;
    ctx.PTR_LOOP_1050_4a06 = 0x0;
    ctx.PTR_LOOP_1050_4a0e = 0x0;
    ctx.PTR_LOOP_1050_4a0c = 0x0;
    ctx.PTR_LOOP_1050_4a14 = 0x0;
    ctx.PTR_LOOP_1050_4a12 = 0x0;
    ctx.PTR_LOOP_1050_4a1a = 0x0;
    ctx.PTR_LOOP_1050_4a18 = 0x0;
    ctx.PTR_LOOP_1050_4a1e = 0x4470;
    ctx.PTR_LOOP_1050_4a20 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4a22 = ctx.PTR_DAT_1050_0004_1050_4478;
    ctx.PTR_LOOP_1050_4a24 = 0x4448;
    ctx.PTR_LOOP_1050_4a26 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4a28 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_4a2a = 0x4448;
    ctx.PTR_LOOP_1050_4a2c = ctx.data_seg;
    ctx.PTR_LOOP_1050_4a2e = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_4a30 = 0x0;
    ctx.PTR_LOOP_1050_4a36 = 0x0;
    ctx.PTR_LOOP_1050_492c = 0xffff;
    ctx.PTR_LOOP_1050_4932 = 0xffff;
    ctx.PTR_LOOP_1050_4938 = 0xffff;
    ctx.PTR_LOOP_1050_494a = 0xffff;
    ctx.PTR_LOOP_1050_4950 = 0xffff;
    ctx.PTR_LOOP_1050_4a34 = 0xffff;
    ctx.PTR_LOOP_1050_4a3a = 0xffff;
    ctx.PTR_LOOP_1050_4a3c = 0x0;
    ctx.PTR_LOOP_1050_4a42 = 0x0;
    ctx.PTR_LOOP_1050_4956 = 0x0;
    ctx.PTR_LOOP_1050_495c = 0x0;
    ctx.PTR_LOOP_1050_4962 = 0x0;
    ctx.PTR_LOOP_1050_4968 = 0x0;
    ctx.PTR_LOOP_1050_496e = 0x0;
    ctx.PTR_LOOP_1050_4974 = 0x0;
    ctx.PTR_LOOP_1050_497a = 0x0;
    ctx.PTR_LOOP_1050_4980 = 0x0;
    ctx.PTR_LOOP_1050_4986 = 0x0;
    ctx.PTR_LOOP_1050_498c = 0x0;
    ctx.PTR_LOOP_1050_4992 = 0x0;
    ctx.PTR_LOOP_1050_4998 = 0x0;
    ctx.PTR_LOOP_1050_4a04 = 0x0;
    ctx.PTR_LOOP_1050_4a0a = 0x0;
    ctx.PTR_LOOP_1050_4a10 = 0x0;
    ctx.PTR_LOOP_1050_4a16 = 0x0;
    ctx.PTR_LOOP_1050_4a1c = 0x0;
    ctx.PTR_LOOP_1050_4a40 = 0x0;
    ctx.PTR_LOOP_1050_4a46 = 0x0;
    ctx.PTR_LOOP_1050_4a48 = 0x4448;
    ctx.PTR_LOOP_1050_4a4a = ctx.data_seg;
    ctx.PTR_LOOP_1050_4a4c = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_4a4e = 0x4448;
    ctx.PTR_LOOP_1050_4a50 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4a52 = ctx.DAT_1050_4452;
    ctx.PTR_LOOP_1050_4a54 = 0x0;
    ctx.PTR_LOOP_1050_4a5a = 0x0;
    ctx.PTR_LOOP_1050_4a60 = 0x0;
    ctx.PTR_LOOP_1050_4a66 = 0x0;
    ctx.PTR_LOOP_1050_4a6c = 0x0;
    ctx.PTR_LOOP_1050_4a72 = 0x0;
    ctx.PTR_LOOP_1050_4a78 = 0x0;
    ctx.PTR_LOOP_1050_4a7e = 0x0;
    ctx.PTR_LOOP_1050_4a84 = 0x0;
    ctx.PTR_LOOP_1050_4a8a = 0x0;
    ctx.PTR_LOOP_1050_4a90 = 0x0;
    ctx.PTR_LOOP_1050_4a96 = 0x0;
    ctx.PTR_LOOP_1050_4a9c = 0x0;
    ctx.PTR_LOOP_1050_4aa2 = 0x0;
    ctx.PTR_LOOP_1050_4aa8 = 0x0;
    ctx.PTR_LOOP_1050_4aae = 0x0;
    ctx.PTR_LOOP_1050_4ab4 = 0x0;
    ctx.PTR_LOOP_1050_4aba = 0x0;
    ctx.PTR_LOOP_1050_4ac0 = 0x0;
    ctx.PTR_LOOP_1050_4ac6 = 0x0;
    ctx.PTR_LOOP_1050_4acc = 0x0;
    ctx.PTR_LOOP_1050_4ad2 = 0x0;
    ctx.PTR_LOOP_1050_4ad8 = 0x0;
    ctx.PTR_LOOP_1050_4ade = 0x0;
    ctx.PTR_LOOP_1050_4ae4 = 0x0;
    ctx.PTR_LOOP_1050_4aea = 0x0;
    ctx.PTR_LOOP_1050_4af0 = 0x0;
    ctx.PTR_LOOP_1050_4a58 = 0x0;
    ctx.PTR_LOOP_1050_4a5e = 0x0;
    ctx.PTR_LOOP_1050_4a64 = 0x0;
    ctx.PTR_LOOP_1050_4a6a = 0x0;
    ctx.PTR_LOOP_1050_4a70 = 0x0;
    ctx.PTR_LOOP_1050_4a76 = 0x0;
    ctx.PTR_LOOP_1050_4a7c = 0x0;
    ctx.PTR_LOOP_1050_4a82 = 0x0;
    ctx.PTR_LOOP_1050_4a88 = 0x0;
    ctx.PTR_LOOP_1050_4a8e = 0x0;
    ctx.PTR_LOOP_1050_4a94 = 0x0;
    ctx.PTR_LOOP_1050_4a9a = 0x0;
    ctx.PTR_LOOP_1050_4aa0 = 0x0;
    ctx.PTR_LOOP_1050_4aa6 = 0x0;
    ctx.PTR_LOOP_1050_4aac = 0x0;
    ctx.PTR_LOOP_1050_4ab2 = 0x0;
    ctx.PTR_LOOP_1050_4ab8 = 0x0;
    ctx.PTR_LOOP_1050_4abe = 0x0;
    ctx.PTR_LOOP_1050_4ac4 = 0x0;
    ctx.PTR_LOOP_1050_4aca = 0x0;
    ctx.PTR_LOOP_1050_4ad0 = 0x0;
    ctx.PTR_LOOP_1050_4ad6 = 0x0;
    ctx.PTR_LOOP_1050_4adc = 0x0;
    ctx.PTR_LOOP_1050_4ae2 = 0x0;
    ctx.PTR_LOOP_1050_4ae8 = 0x0;
    ctx.PTR_LOOP_1050_4aee = 0x0;
    ctx.PTR_LOOP_1050_4af4 = 0x0;
    pu_var3 = 0x4af6;
    // TODO: refactor for loop
    // for (i_var2 = 0x1b; i_var2 != 0x0; i_var2 += -0x1) {
    //   pu_var1 = pu_var3;
    //   pu_var3 = pu_var3 + 0x1;
    //   *pu_var1 = 0x0;
    // }
    ctx.PTR_LOOP_1050_4b9c = ctx.PTR_LOOP_1050_4434;
    ctx.PTR_LOOP_1050_4b9e = 0x0;
    ctx.PTR_LOOP_1050_4ba4 = 0x0;
    ctx.PTR_LOOP_1050_4baa = 0x0;
    ctx.PTR_LOOP_1050_4ba2 = 0xffff;
    ctx.PTR_LOOP_1050_4ba8 = 0xffff;
    ctx.PTR_LOOP_1050_4bae = 0xffff;
    ctx.PTR_LOOP_1050_4bb0 = 0x0;
    ctx.PTR_LOOP_1050_4bb6 = 0x0;
    ctx.PTR_LOOP_1050_4bbc = 0x448e;
    ctx.PTR_LOOP_1050_4bbe = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bc2 = 0x448e;
    ctx.PTR_LOOP_1050_4bc4 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bc0 = ctx.DAT_1050_4494;
    ctx.PTR_LOOP_1050_4bc6 = ctx.DAT_1050_4494;
    ctx.PTR_LOOP_1050_4bc8 = 0x448e;
    ctx.PTR_LOOP_1050_4bca = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bcc = ctx.DAT_1050_4494;
    ctx.PTR_LOOP_1050_4bce = 0x4482;
    ctx.PTR_LOOP_1050_4bd0 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bd4 = 0x4482;
    ctx.PTR_LOOP_1050_4bd6 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bd2 = ctx.PTR_LOOP_1050_4486;
    ctx.PTR_LOOP_1050_4bd8 = ctx.PTR_LOOP_1050_4486;
    ctx.PTR_LOOP_1050_4bda = 0x4482;
    ctx.PTR_LOOP_1050_4bdc = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bde = ctx.PTR_LOOP_1050_4486;
    ctx.PTR_LOOP_1050_4be2 = 0x0;
    ctx.PTR_LOOP_1050_4be0 = 0x0;
    ctx.PTR_LOOP_1050_4bb4 = 0x0;
    ctx.PTR_LOOP_1050_4bba = 0x0;
    ctx.PTR_LOOP_1050_4be4 = 0x0;
    ctx.PTR_LOOP_1050_4be6 = 0x44ac;
    ctx.PTR_LOOP_1050_4be8 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bec = 0x44ac;
    ctx.PTR_LOOP_1050_4bee = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bea = ctx.DAT_1050_44b2;
    ctx.PTR_LOOP_1050_4bf0 = ctx.DAT_1050_44b2;
    ctx.PTR_LOOP_1050_4bf2 = 0x44ac;
    ctx.PTR_LOOP_1050_4bf4 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bf6 = ctx.DAT_1050_44b2;
    ctx.PTR_LOOP_1050_4bf8 = 0x446a;
    ctx.PTR_LOOP_1050_4bfa = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bfe = 0x446a;
    ctx.PTR_LOOP_1050_4c00 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4bfc = ctx.PTR_LOOP_1050_446e;
    ctx.PTR_LOOP_1050_4c02 = ctx.PTR_LOOP_1050_446e;
    ctx.PTR_LOOP_1050_4c04 = 0x446a;
    ctx.PTR_LOOP_1050_4c06 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4c08 = ctx.PTR_LOOP_1050_446e;
    ctx.PTR_LOOP_1050_4c0a = 0x448e;
    ctx.PTR_LOOP_1050_4c0c = ctx.data_seg;
    ctx.PTR_LOOP_1050_4c0e = ctx.DAT_1050_4494;
    ctx.PTR_LOOP_1050_4c10 = 0x448e;
    ctx.PTR_LOOP_1050_4c12 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4c14 = ctx.DAT_1050_4494;
    ctx.PTR_LOOP_1050_4c16 = 0x44ac;
    ctx.PTR_LOOP_1050_4c18 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4c1a = ctx.DAT_1050_44b2;
    ctx.PTR_LOOP_1050_4c22 = 0x448e;
    ctx.PTR_LOOP_1050_4c24 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4c26 = ctx.DAT_1050_4494;
    ctx.PTR_LOOP_1050_4c28 = 0x0;
    ctx.PTR_LOOP_1050_4c2e = 0x0;
    ctx.PTR_LOOP_1050_4c34 = 0x0;
    ctx.PTR_LOOP_1050_4c3a = 0x0;
    ctx.PTR_LOOP_1050_4c40 = 0x0;
    ctx.PTR_LOOP_1050_4c46 = 0x0;
    ctx.PTR_LOOP_1050_4c4c = 0x0;
    ctx.PTR_LOOP_1050_4c52 = 0x0;
    ctx.PTR_LOOP_1050_4c1c = 0x44ac;
    ctx.PTR_LOOP_1050_4c1e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4c58 = 0x44ac;
    ctx.PTR_LOOP_1050_4c5a = ctx.data_seg;
    ctx.PTR_LOOP_1050_4c20 = ctx.DAT_1050_44b2;
    ctx.PTR_LOOP_1050_4c5c = ctx.DAT_1050_44b2;
    ctx.PTR_LOOP_1050_4c5e = 0x44ac;
    ctx.PTR_LOOP_1050_4c60 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4c62 = ctx.DAT_1050_44b2;
    ctx.PTR_LOOP_1050_4c64 = 0x0;
    ctx.PTR_LOOP_1050_4c6a = 0x0;
    ctx.PTR_LOOP_1050_4c70 = 0x0;
    ctx.PTR_LOOP_1050_4c76 = 0x446a;
    ctx.PTR_LOOP_1050_4c78 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4c7c = 0x446a;
    ctx.PTR_LOOP_1050_4c7e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4c7a = ctx.PTR_LOOP_1050_446e;
    ctx.PTR_LOOP_1050_4c80 = ctx.PTR_LOOP_1050_446e;
    ctx.PTR_LOOP_1050_4c82 = 0x0;
    ctx.PTR_LOOP_1050_4c88 = 0x0;
    ctx.PTR_LOOP_1050_4c2c = 0xffff;
    ctx.PTR_LOOP_1050_4c32 = 0xffff;
    ctx.PTR_LOOP_1050_4c38 = 0xffff;
    ctx.PTR_LOOP_1050_4c3e = 0xffff;
    ctx.PTR_LOOP_1050_4c44 = 0xffff;
    ctx.PTR_LOOP_1050_4c4a = 0xffff;
    ctx.PTR_LOOP_1050_4c68 = 0xffff;
    ctx.PTR_LOOP_1050_4c6e = 0xffff;
    ctx.PTR_LOOP_1050_4c74 = 0xffff;
    ctx.PTR_LOOP_1050_4c86 = 0xffff;
    ctx.PTR_LOOP_1050_4c8c = 0xffff;
    ctx.PTR_LOOP_1050_4c8e = 0x0;
    ctx.PTR_LOOP_1050_4c94 = 0x0;
    ctx.PTR_LOOP_1050_4c9a = 0x0;
    ctx.PTR_LOOP_1050_4ca0 = 0x0;
    ctx.PTR_LOOP_1050_4ca6 = 0x0;
    ctx.PTR_LOOP_1050_4cac = 0x0;
    ctx.PTR_LOOP_1050_4cb2 = 0x0;
    ctx.PTR_LOOP_1050_4cb8 = 0x0;
    ctx.PTR_LOOP_1050_4cbe = 0x0;
    ctx.PTR_LOOP_1050_4cc4 = 0x0;
    ctx.PTR_LOOP_1050_4cca = 0x0;
    ctx.PTR_LOOP_1050_4cd0 = 0x0;
    ctx.PTR_LOOP_1050_4cd6 = 0x4496;
    ctx.PTR_LOOP_1050_4cd8 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4cdc = 0x4496;
    ctx.PTR_LOOP_1050_4cde = ctx.data_seg;
    ctx.PTR_LOOP_1050_4cda = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4ce0 = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4ce2 = 0x4496;
    ctx.PTR_LOOP_1050_4ce4 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4ce6 = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4ce8 = 0x4496;
    ctx.PTR_LOOP_1050_4cea = ctx.data_seg;
    ctx.PTR_LOOP_1050_4cec = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4cee = 0x4496;
    ctx.PTR_LOOP_1050_4cf0 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4cf2 = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4cf4 = 0x44a4;
    ctx.PTR_LOOP_1050_4cf6 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4cfa = 0x44a4;
    ctx.PTR_LOOP_1050_4cfc = ctx.data_seg;
    ctx.PTR_LOOP_1050_4cf8 = ctx.DAT_1050_44aa;
    ctx.PTR_LOOP_1050_4cfe = ctx.DAT_1050_44aa;
    ctx.PTR_LOOP_1050_4d00 = 0x44a4;
    ctx.PTR_LOOP_1050_4d02 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d04 = ctx.DAT_1050_44aa;
    ctx.PTR_LOOP_1050_4d06 = 0x4496;
    ctx.PTR_LOOP_1050_4d08 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d0a = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4d0c = 0x4496;
    ctx.PTR_LOOP_1050_4d0e = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d10 = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4d12 = 0x4496;
    ctx.PTR_LOOP_1050_4d14 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d16 = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4d18 = 0x4496;
    ctx.PTR_LOOP_1050_4d1a = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d1c = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4d1e = 0x4482;
    ctx.PTR_LOOP_1050_4d20 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d24 = 0x4482;
    ctx.PTR_LOOP_1050_4d26 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d22 = ctx.PTR_LOOP_1050_4486;
    ctx.PTR_LOOP_1050_4d28 = ctx.PTR_LOOP_1050_4486;
    ctx.PTR_LOOP_1050_4d2a = 0x44a4;
    ctx.PTR_LOOP_1050_4d2c = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d2e = ctx.DAT_1050_44aa;
    ctx.PTR_LOOP_1050_4d30 = 0x44a4;
    ctx.PTR_LOOP_1050_4d32 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d34 = ctx.DAT_1050_44aa;
    ctx.PTR_LOOP_1050_4d36 = 0x44a4;
    ctx.PTR_LOOP_1050_4d38 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d3a = ctx.DAT_1050_44aa;
    ctx.PTR_LOOP_1050_4d3c = 0x0;
    ctx.PTR_LOOP_1050_4d42 = 0x0;
    ctx.PTR_LOOP_1050_4c50 = 0x0;
    ctx.PTR_LOOP_1050_4c56 = 0x0;
    ctx.PTR_LOOP_1050_4c92 = 0x0;
    ctx.PTR_LOOP_1050_4c98 = 0x0;
    ctx.PTR_LOOP_1050_4c9e = 0x0;
    ctx.PTR_LOOP_1050_4ca4 = 0x0;
    ctx.PTR_LOOP_1050_4caa = 0x0;
    ctx.PTR_LOOP_1050_4cb0 = 0x0;
    ctx.PTR_LOOP_1050_4cb6 = 0x0;
    ctx.PTR_LOOP_1050_4cbc = 0x0;
    ctx.PTR_LOOP_1050_4cc2 = 0x0;
    ctx.PTR_LOOP_1050_4cc8 = 0x0;
    ctx.PTR_LOOP_1050_4cce = 0x0;
    ctx.PTR_LOOP_1050_4cd4 = 0x0;
    ctx.PTR_LOOP_1050_4d40 = 0x0;
    ctx.PTR_LOOP_1050_4d46 = 0x0;
    ctx.PTR_LOOP_1050_4d48 = 0x0;
    ctx.PTR_LOOP_1050_4d4e = 0x0;
    ctx.PTR_LOOP_1050_4d54 = 0x0;
    ctx.PTR_LOOP_1050_4d5a = 0x44a4;
    ctx.PTR_LOOP_1050_4d5c = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d5e = ctx.DAT_1050_44aa;
    ctx.PTR_LOOP_1050_4d60 = 0x4496;
    ctx.PTR_LOOP_1050_4d62 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d66 = 0x4496;
    ctx.PTR_LOOP_1050_4d68 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d64 = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4d6a = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4d6c = 0x0;
    ctx.PTR_LOOP_1050_4d72 = 0x0;
    ctx.PTR_LOOP_1050_4d70 = 0xffff;
    ctx.PTR_LOOP_1050_4d76 = 0xffff;
    ctx.PTR_LOOP_1050_4d78 = 0x0;
    ctx.PTR_LOOP_1050_4d7e = 0x0;
    ctx.PTR_LOOP_1050_4d84 = 0x4496;
    ctx.PTR_LOOP_1050_4d86 = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d88 = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4d8a = 0x4496;
    ctx.PTR_LOOP_1050_4d8c = ctx.data_seg;
    ctx.PTR_LOOP_1050_4d8e = ctx.DAT_1050_44a2;
    ctx.PTR_LOOP_1050_4d90 = 0x0;
    ctx.PTR_LOOP_1050_4d96 = 0x0;
    ctx.PTR_LOOP_1050_4d9c = 0x0;
    ctx.PTR_LOOP_1050_4da2 = 0x0;
    ctx.PTR_LOOP_1050_4da8 = 0x0;
    ctx.PTR_LOOP_1050_4dae = 0x0;
    ctx.PTR_LOOP_1050_4db4 = 0x0;
    ctx.PTR_LOOP_1050_4dba = 0x0;
    ctx.PTR_LOOP_1050_4dc0 = 0x0;
    ctx.PTR_LOOP_1050_4dc6 = 0x0;
    ctx.PTR_LOOP_1050_4dcc = 0x0;
    ctx.PTR_LOOP_1050_4dd2 = 0x0;
    ctx.PTR_LOOP_1050_4dd8 = 0x0;
    ctx.PTR_LOOP_1050_4dde = 0x0;
    ctx.PTR_LOOP_1050_4de4 = 0x0;
    ctx.PTR_LOOP_1050_4dea = 0x0;
    ctx.PTR_LOOP_1050_4df0 = 0x0;
    ctx.PTR_LOOP_1050_4df6 = 0x0;
    ctx.PTR_LOOP_1050_4dfc = 0x0;
    ctx.PTR_LOOP_1050_4e02 = 0x0;
    ctx.PTR_LOOP_1050_4e08 = 0x0;
    ctx.PTR_LOOP_1050_4e0e = 0x0;
    ctx.PTR_LOOP_1050_4e14 = 0x0;
    ctx.PTR_LOOP_1050_4e1a = 0x0;
    ctx.PTR_LOOP_1050_4e20 = 0x0;
    ctx.PTR_LOOP_1050_4e26 = 0x0;
    ctx.PTR_LOOP_1050_4e2c = 0x0;
    ctx.PTR_LOOP_1050_4d4c = 0x0;
    ctx.PTR_LOOP_1050_4d52 = 0x0;
    ctx.PTR_LOOP_1050_4d58 = 0x0;
    ctx.PTR_LOOP_1050_4d7c = 0x0;
    ctx.PTR_LOOP_1050_4d82 = 0x0;
    ctx.PTR_LOOP_1050_4d94 = 0x0;
    ctx.PTR_LOOP_1050_4d9a = 0x0;
    ctx.PTR_LOOP_1050_4da0 = 0x0;
    ctx.PTR_LOOP_1050_4da6 = 0x0;
    ctx.PTR_LOOP_1050_4dac = 0x0;
    ctx.PTR_LOOP_1050_4db2 = 0x0;
    ctx.PTR_LOOP_1050_4db8 = 0x0;
    ctx.PTR_LOOP_1050_4dbe = 0x0;
    ctx.PTR_LOOP_1050_4dc4 = 0x0;
    ctx.PTR_LOOP_1050_4dca = 0x0;
    ctx.PTR_LOOP_1050_4dd0 = 0x0;
    ctx.PTR_LOOP_1050_4dd6 = 0x0;
    ctx.PTR_LOOP_1050_4ddc = 0x0;
    ctx.PTR_LOOP_1050_4de2 = 0x0;
    ctx.PTR_LOOP_1050_4de8 = 0x0;
    ctx.PTR_LOOP_1050_4dee = 0x0;
    ctx.PTR_LOOP_1050_4df4 = 0x0;
    ctx.PTR_LOOP_1050_4dfa = 0x0;
    ctx.PTR_LOOP_1050_4e00 = 0x0;
    ctx.PTR_LOOP_1050_4e06 = 0x0;
    ctx.PTR_LOOP_1050_4e0c = 0x0;
    ctx.PTR_LOOP_1050_4e12 = 0x0;
    ctx.PTR_LOOP_1050_4e18 = 0x0;
    ctx.PTR_LOOP_1050_4e1e = 0x0;
    ctx.PTR_LOOP_1050_4e24 = 0x0;
    ctx.PTR_LOOP_1050_4e2a = 0x0;
    ctx.PTR_LOOP_1050_4e30 = 0x0;
    pu_var3 = 0x4e32;
    // TODO: refactor for loop
    // for (i_var2 = 0x1b; i_var2 != 0x0; i_var2 += -0x1) {
    //   pu_var1 = pu_var3;
    //   pu_var3 = pu_var3 + 0x1;
    //   *pu_var1 = 0x0;
    // }
    return;
}

pub unsafe fn init_op_1008_54aa(
    ctx: &mut AppContext,
    param_1: U32Ptr,
    param_2: &mut String,
    param_3: U32Ptr,
    param_4: U32Ptr,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: &mut String,
    in_cx: u16,
    in_dx: &mut u16,
    stack0xfffe: &mut i16,
) {
    let ppc_var1: u32;
    let u_var3: u16;
    let pu_var4: u32;
    let mut extraout_dx: u16 = 0;
    let u_var5: u16;
    let mut extraout_dx_00: u16 = 0;
    let u_var6: u16;
    let mut extraout_dx_01: u16 = 0;
    let u_var7: u32;
    let pu_stack12: u32;
    let u_var2: u32;

    if param_3.is_null() == false {
        return;
    }

    let param_1 = make_u16_ptr(0);

    dos3_call_op_1000_435c(param_1, in_cx, in_dx, stack0xfffe, param_8);
    pass1_1000_4d0c(ctx, param_5);
    pass1_1000_1fea(ctx);
    let mut param_1: u16 = 0;
    ctx.PTR_LOOP_1050_03a0 = mem_op_1000_1902(ctx, &mut param_1, 0x32, 0x0, 0x12, 0x1000, in_dx);
    ctx.PTR_LOOP_1050_029c = mem_op_1000_1902(
        ctx,
        &mut param_1,
        0x64,
        0x0,
        0xc,
        0x1000,
        (ctx.PTR_LOOP_1050_03a0 >> 0x10),
    );
    ctx.PTR_LOOP_1050_4fb8 = mem_op_1000_1902(
        ctx,
        &mut param_1,
        0x64,
        0x0,
        0x10,
        0x1000,
        (ctx.PTR_LOOP_1050_029c >> 0x10),
    );
    ctx.PTR_LOOP_1050_68a2 = mem_op_1000_1902(
        ctx,
        &mut param_1,
        0x64,
        0x0,
        0xe,
        0x1000,
        (ctx.PTR_LOOP_1050_4fb8 >> 0x10),
    );
    ctx.PTR_LOOP_1050_5744 = mem_op_1000_1902(
        ctx,
        &mut param_1,
        0x1f4,
        0x0,
        0x42,
        0x1000,
        (ctx.PTR_LOOP_1050_68a2 >> 0x10),
    );
    u_var7 = mem_op_1000_1902(
        ctx,
        &mut param_1,
        0x32,
        0x0,
        0x6,
        0x1000,
        (ctx.PTR_LOOP_1050_5744 >> 0x10),
    );
    pu_var4 = u_var7;
    ctx.PTR_LOOP_1050_5768 = u_var7;
    ctx.PTR_LOOP_1050_038c = param_4;
    ctx.PTR_LOOP_1050_038e = param_3;
    ctx.PTR_LOOP_1050_0390 = param_1;
    ctx.PTR_LOOP_1050_576a = pu_var4;
    u_var3 = str_op_1008_60e8(ctx, param_2, pu_var4 as u16);
    ctx.PTR_LOOP_1050_0392 = CONCAT22(*pu_var4 as u16, u_var3);
    mem_op_1000_179c(ctx, 0xc, pu_var4 as u16, 0x1000);
    if (pu_var4 | u_var3) == 0x0 {
        u_var3 = 0x0;
        u_var5 = 0x0;
    } else {
        struct_op_1008_0000(CONCAT13(
            (pu_var4 >> 0x8) as u16,
            CONCAT12(*pu_var4, u_var3),
        ));
        u_var5 = extraout_dx;
    }
    pu_stack12 = CONCAT22(u_var5, u_var3);
    if ctx.PTR_LOOP_1050_0392 != 0x0 {
        ppc_var1 = (*pu_stack12 + 0x4);
        (**ppc_var1)(
            0x1000,
            u_var3,
            u_var5,
            ctx.PTR_LOOP_1050_0392,
            (ctx.PTR_LOOP_1050_0392 >> 0x10),
        );
    }
    u_var2 = *pu_stack12;
    ppc_var1 = u_var2 + 0x4;
    (**ppc_var1)(0x1000, u_var3, u_var5);
    u_var6 = extraout_dx_00;
    win_msg_op_1008_9498(&ctx.PTR_LOOP_1050_1000, param_8);
    if pu_stack12 != 0x0 {
        ppc_var1 = u_var2;
        (**ppc_var1)(0x1000, u_var3, u_var5, 0x1);
        u_var6 = extraout_dx_01;
    }
    u_var7 = mem_op_1000_1b68(
        u_var6,
        0x1000,
        ctx.PTR_LOOP_1050_03a0,
        (ctx.PTR_LOOP_1050_03a0 >> 0x10),
    );
    u_var7 = mem_op_1000_1b68(
        (u_var7 >> 0x10) as u16,
        0x1000,
        ctx.PTR_LOOP_1050_029c,
        (ctx.PTR_LOOP_1050_029c >> 0x10),
    );
    u_var7 = mem_op_1000_1b68(
        (u_var7 >> 0x10) as u16,
        0x1000,
        ctx.PTR_LOOP_1050_4fb8,
        (ctx.PTR_LOOP_1050_4fb8 >> 0x10),
    );
    u_var7 = mem_op_1000_1b68(
        (u_var7 >> 0x10) as u16,
        0x1000,
        ctx.PTR_LOOP_1050_68a2,
        (ctx.PTR_LOOP_1050_68a2 >> 0x10),
    );
    mem_op_1000_1b68(
        (u_var7 >> 0x10) as u16,
        0x1000,
        ctx.PTR_LOOP_1050_5744,
        (ctx.PTR_LOOP_1050_5744 >> 0x10),
    );
    return;
}
