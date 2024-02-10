slint::include_modules!();

const TAX_PER: f64 = 0.30;
const OWNER_PER: f64 = 0.55;
const PROFIT_PER: f64 = 0.05;
const OPERATIONAL_EXPENSES_PER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |s| {
        let ui = ui_handle.unwrap();
        let num = s.parse::<f64>().or::<f64>(Ok(0.0)).unwrap();

        let tax = num * TAX_PER;
        let owner = num * OWNER_PER;
        let profit = num * PROFIT_PER;
        let op_ex = num * OPERATIONAL_EXPENSES_PER;

        let result = format!(
            "Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOp.Ex: {:.2}\n",
            tax, owner, profit, op_ex
        );

        ui.set_results(result.into());
    });

    ui.run()
}
