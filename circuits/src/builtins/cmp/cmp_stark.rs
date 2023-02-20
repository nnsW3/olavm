use crate::builtins::cmp::columns::*;
use itertools::Itertools;

use crate::stark::constraint_consumer::{ConstraintConsumer, RecursiveConstraintConsumer};
use crate::stark::cross_table_lookup::Column;
use crate::stark::stark::Stark;
use crate::stark::vars::{StarkEvaluationTargets, StarkEvaluationVars};
use plonky2::field::extension::{Extendable, FieldExtension};
use plonky2::field::packed::PackedField;
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use std::marker::PhantomData;

#[derive(Copy, Clone, Default)]
pub struct CmpStark<F, const D: usize> {
    pub _phantom: PhantomData<F>,
}
impl<F: RichField + Extendable<D>, const D: usize> Stark<F, D> for CmpStark<F, D> {
    const COLUMNS: usize = COL_NUM_CMP;
    fn eval_packed_generic<FE, P, const D2: usize>(
        &self,
        vars: StarkEvaluationVars<FE, P, { COL_NUM_CMP }>,
        yield_constr: &mut ConstraintConsumer<P>,
    ) where
        FE: FieldExtension<D2, BaseField = F>,
        P: PackedField<Scalar = FE>,
    {
        let op0 = vars.local_values[COL_CMP_OP0];
        let op1 = vars.local_values[COL_CMP_OP1];
        let gte = vars.local_values[COL_CMP_GTE];
        let abs_diff = vars.local_values[COL_CMP_ABS_DIFF];
        let abs_diff_inv = vars.local_values[COL_CMP_ABS_DIFF_INV];

        // gte must be binary
        yield_constr.constraint(gte * (P::ONES - gte));
        // abs_diff calculation
        yield_constr.constraint(gte * (op0 - op1 - abs_diff));
        yield_constr.constraint((P::ONES - gte) * (op1 - op0 - abs_diff));
        // abs_diff * abs_diff_inv = 1 when gte = 0
        yield_constr.constraint((P::ONES - gte) * (P::ONES - abs_diff * abs_diff_inv));
    }

    fn eval_ext_circuit(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        vars: StarkEvaluationTargets<D, { COL_NUM_CMP }>,
        yield_constr: &mut RecursiveConstraintConsumer<F, D>,
    ) {
        let one = builder.one_extension();
        let op0 = vars.local_values[COL_CMP_OP0];
        let op1 = vars.local_values[COL_CMP_OP1];
        let gte = vars.local_values[COL_CMP_GTE];
        let abs_diff = vars.local_values[COL_CMP_ABS_DIFF];
        let abs_diff_inv = vars.local_values[COL_CMP_ABS_DIFF_INV];

        // gte must be binary
        let one_m_gte = builder.sub_extension(one, gte);
        let gte_binary_cs = builder.mul_extension(gte, one_m_gte);
        yield_constr.constraint(builder, gte_binary_cs);
        // abs_diff calculation
        let op1_add_diff = builder.add_extension(op1, abs_diff);
        let op0_add_diff = builder.add_extension(op0, abs_diff);
        let op0_m_sum = builder.sub_extension(op0, op1_add_diff);
        let op1_m_sum = builder.sub_extension(op1, op0_add_diff);
        let diff_gte_cs = builder.mul_extension(gte, op0_m_sum);
        let diff_lt_cs = builder.mul_extension(one_m_gte, op1_m_sum);
        yield_constr.constraint(builder, diff_gte_cs);
        yield_constr.constraint(builder, diff_lt_cs);
        // abs_diff * abs_diff_inv = 1 when gte = 0
        let diff_mul = builder.mul_extension(abs_diff, abs_diff_inv);
        let one_m_diff_mul = builder.sub_extension(one, diff_mul);
        let inv_cs = builder.mul_extension(one_m_gte, one_m_diff_mul);
        yield_constr.constraint(builder, inv_cs);
    }

    fn constraint_degree(&self) -> usize {
        3
    }
}

// Get the column info for Cross_Lookup<Cpu_table, Bitwise_table>
pub fn ctl_data_with_rangecheck<F: Field>() -> Vec<Column<F>> {
    Column::singles([COL_CMP_ABS_DIFF]).collect_vec()
}

pub fn ctl_filter_with_rangecheck<F: Field>() -> Column<F> {
    Column::single(COL_CMP_FILTER_LOOKING_RC)
}

// Get the column info for Cross_Lookup<Cpu_table, Bitwise_table>
pub fn ctl_data_with_cpu<F: Field>() -> Vec<Column<F>> {
    Column::singles([COL_CMP_OP0, COL_CMP_OP1, COL_CMP_GTE]).collect_vec()
}

pub fn ctl_filter_with_cpu<F: Field>() -> Column<F> {
    Column::single(COL_CMP_FILTER_LOOKING_RC)
}
